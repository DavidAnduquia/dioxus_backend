use std::fs::{self, OpenOptions};
use std::io::Write;
use std::sync::{Mutex, OnceLock};
use std::collections::VecDeque;
use chrono::{Local, FixedOffset, TimeZone, Utc};
use tracing_subscriber::{fmt::writer::MakeWriter, layer::SubscriberExt, EnvFilter, fmt, util::SubscriberInitExt};

// Zona horaria de Bogotá (UTC-5)
struct BogotaTime;

impl tracing_subscriber::fmt::time::FormatTime for BogotaTime {
    fn format_time(&self, w: &mut tracing_subscriber::fmt::format::Writer<'_>) -> std::fmt::Result {
        let bogota_tz = FixedOffset::west_opt(5 * 3600).unwrap();
        let now_bogota = bogota_tz.from_utc_datetime(&Utc::now().naive_utc());
        write!(w, "{}", now_bogota.format("%Y-%m-%d %H:%M:%S%.3f"))
    }
}

// Buffer circular con límite de 20KB para optimizar memoria
const MAX_BUFFER_SIZE: usize = 20 * 1024; // 20KB
const FLUSH_THRESHOLD: usize = 4 * 1024;  // Flush cada 4KB

struct CircularLogBuffer {
    buffer: VecDeque<u8>,
    log_dir: &'static str,
    app_name: &'static str,
    bytes_written: usize,
}

impl CircularLogBuffer {
    fn new(log_dir: &'static str, app_name: &'static str) -> Self {
        Self {
            buffer: VecDeque::with_capacity(FLUSH_THRESHOLD),
            log_dir,
            app_name,
            bytes_written: 0,
        }
    }

    fn write_to_buffer(&mut self, data: &[u8]) -> std::io::Result<usize> {
        // Si el buffer excede el límite, hacer flush
        if self.buffer.len() + data.len() > MAX_BUFFER_SIZE {
            self.flush_to_disk()?;
        }

        // Agregar datos al buffer
        self.buffer.extend(data);
        self.bytes_written += data.len();

        // Flush automático cada FLUSH_THRESHOLD bytes
        if self.buffer.len() >= FLUSH_THRESHOLD {
            self.flush_to_disk()?;
        }

        Ok(data.len())
    }

    fn flush_to_disk(&mut self) -> std::io::Result<()> {
        if self.buffer.is_empty() {
            return Ok(());
        }

        // Crear directorio si no existe
        std::fs::create_dir_all(self.log_dir).ok();

        // Zona horaria de Bogotá (UTC-5)
        let bogota_tz = FixedOffset::west_opt(5 * 3600).unwrap();
        let now_bogota = bogota_tz.from_utc_datetime(&chrono::Utc::now().naive_utc());
        
        // Nombre del archivo con fecha actual (zona horaria Bogotá)
        let log_file = format!(
            "{}/{}.{}.log",
            self.log_dir,
            self.app_name,
            now_bogota.format("%Y-%m-%d")
        );

        // Abrir archivo en modo append, escribir y cerrar inmediatamente
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_file)?;

        // Escribir todo el buffer de una vez
        let data: Vec<u8> = self.buffer.drain(..).collect();
        file.write_all(&data)?;
        file.flush()?;
        // Archivo se cierra automáticamente al salir del scope

        Ok(())
    }
}

static LOG_BUFFER: OnceLock<Mutex<CircularLogBuffer>> = OnceLock::new();

#[derive(Clone)]
struct BufferedMakeWriter {
    _private: (),
}

struct BufferedWriter;

impl Write for BufferedWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match LOG_BUFFER.get() {
            Some(buffer) => {
                match buffer.lock() {
                    Ok(mut b) => b.write_to_buffer(buf),
                    Err(_) => std::io::stderr().write(buf),
                }
            }
            None => std::io::stderr().write(buf),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match LOG_BUFFER.get() {
            Some(buffer) => {
                match buffer.lock() {
                    Ok(mut b) => b.flush_to_disk(),
                    Err(_) => std::io::stderr().flush(),
                }
            }
            None => std::io::stderr().flush(),
        }
    }
}

impl BufferedMakeWriter {
    fn new() -> Self {
        Self { _private: () }
    }
}

impl<'a> MakeWriter<'a> for BufferedMakeWriter {
    type Writer = BufferedWriter;

    fn make_writer(&'a self) -> Self::Writer {
        BufferedWriter
    }
}

/// Inicializa el sistema de logging SOLO a consola (sin archivo)
/// Esto reduce el consumo de memoria significativamente (~50KB menos)
/// 
/// # Returns
/// * `Result<(), Box<dyn std::error::Error>>` - Ok si se inicializó correctamente
/// 
/// # Uso
/// Ideal para desarrollo donde no necesitas persistencia de logs
pub fn init_logger_console_only() -> Result<(), Box<dyn std::error::Error>> {
    static INIT: OnceLock<()> = OnceLock::new();
    
    INIT.get_or_init(|| {
        let env_filter = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("info"));

        // Solo consola, sin archivo ni buffer, con hora Bogotá
        tracing_subscriber::registry()
            .with(env_filter)
            .with(fmt::layer().with_timer(BogotaTime).compact())
            .init();
    });

    Ok(())
}

/// Inicializa el sistema de logging con buffer circular de 20KB
/// 
/// # Arguments
/// * `log_dir` - Directorio donde se guardarán los logs
/// * `app_name` - Nombre de la aplicación para los archivos de log
/// 
/// # Returns
/// * `Result<(), Box<dyn std::error::Error>>` - Ok si se inicializó correctamente
/// 
/// # Optimizaciones
/// - Buffer circular de 20KB máximo en memoria
/// - Escritura lazy (flush cada 4KB o al cerrar)
/// - Archivo se abre/cierra en cada flush (sin mantener handle abierto)
/// - Logs se persisten correctamente entre reinicios
pub fn init_logger(log_dir: &'static str, app_name: &'static str) -> Result<(), Box<dyn std::error::Error>> {
    static INIT: OnceLock<()> = OnceLock::new();
    
    INIT.get_or_init(|| {
        // Inicializar buffer circular
        LOG_BUFFER.get_or_init(|| Mutex::new(CircularLogBuffer::new(log_dir, app_name)));

        // Configurar filtro de niveles
        let env_filter = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("info"));
        
        // Escritor con buffer circular
        let file_writer = BufferedMakeWriter::new();
        
        // Capa para archivo: SIN colores, formato compacto, hora Bogotá
        let file_layer = fmt::layer()
            .with_writer(file_writer)
            .with_ansi(false)
            .with_target(false)
            .with_timer(BogotaTime)
            .compact();
        
        // Capa para consola: CON colores, hora Bogotá
        let stdout_layer = fmt::layer()
            .with_ansi(true)
            .with_timer(BogotaTime)
            .compact();
        
        // Configurar suscriptor
        tracing_subscriber::registry()
            .with(env_filter)
            .with(file_layer)
            .with(stdout_layer)
            .init();
    });
    
    Ok(())
}

/// Fuerza el flush del buffer de logs al disco
/// Útil antes de shutdown para asegurar que todos los logs se persistan
pub fn flush_logs() -> std::io::Result<()> {
    if let Some(buffer) = LOG_BUFFER.get() {
        if let Ok(mut b) = buffer.lock() {
            b.flush_to_disk()?;
        }
    }
    Ok(())
}

/// Limpia los archivos de log antiguos
/// 
/// # Arguments
/// * `log_dir` - Directorio de logs
/// * `max_age_days` - Número máximo de días de antigüedad para conservar los logs
#[allow(dead_code)]
pub fn cleanup_old_logs(log_dir: &str, max_age_days: u64) -> std::io::Result<()> {
    use std::time::{Duration, SystemTime};

    if !std::path::Path::new(log_dir).exists() {
        return Ok(());
    }

    let mut entries = match fs::read_dir(log_dir) {
        Ok(entries) => entries,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(err) => return Err(err),
    };

    let now = SystemTime::now();
    let max_age = Duration::from_secs(max_age_days.saturating_mul(24 * 3600));

    while let Some(entry) = entries.next() {
        let entry = entry?;
        let path = entry.path();

        let file_type = match entry.file_type() {
            Ok(ft) => ft,
            Err(_) => continue,
        };

        if !file_type.is_file() {
            continue;
        }

        let metadata = match entry.metadata() {
            Ok(meta) => meta,
            Err(_) => continue,
        };

        let modified = match metadata.modified() {
            Ok(modified) => modified,
            Err(_) => continue,
        };

        if let Ok(elapsed) = now.duration_since(modified) {
            if elapsed > max_age {
                tracing::info!("🗑️  Eliminando log antiguo: {:?}", path);
                // Ignore errors so a single failure doesn't abort cleanup.
                let _ = fs::remove_file(&path);
            }
        }
    }

    Ok(())
}
