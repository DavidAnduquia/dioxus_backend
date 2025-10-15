use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::sync::{Arc, Mutex, OnceLock};
use chrono::Local;
use tracing_subscriber::{fmt::writer::MakeWriter, layer::SubscriberExt, EnvFilter, fmt, util::SubscriberInitExt};

// Usamos un singleton global para el archivo de log
static LOG_FILE: OnceLock<Arc<Mutex<File>>> = OnceLock::new();

#[derive(Clone)]
struct LockedMakeWriter {
    // Mantenemos la referencia al archivo a través de Arc<Mutex<File>>
    _private: (), // Campo privado para forzar el uso de new()
}
struct LockedWriter;

impl Write for LockedWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        // Try to write to file first, fallback to stderr
        match get_or_init_log_file().lock() {
            Ok(mut file) => file.write(buf),
            Err(_) => std::io::stderr().write(buf)
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        // Try to flush the file, fallback to stderr
        match get_or_init_log_file().lock() {
            Ok(mut file) => file.flush(),
            Err(_) => std::io::stderr().flush()
        }
    }
}

impl LockedMakeWriter {
    fn new() -> Self {
        Self { _private: () }
    }
}

impl<'a> MakeWriter<'a> for LockedMakeWriter {
    type Writer = LockedWriter;

    fn make_writer(&'a self) -> Self::Writer {
        LockedWriter {}
    }
}

// Función para obtener o inicializar el archivo de log
fn get_or_init_log_file() -> Arc<Mutex<File>> {
    LOG_FILE.get_or_init(|| {
        // Configuración por defecto si no se ha inicializado el logger
        let log_dir = "logs";
        let app_name = "app";
        
        // Crear directorio de logs si no existe
        std::fs::create_dir_all(log_dir).unwrap_or_else(|_| {
            eprintln!("No se pudo crear el directorio de logs");
        });

        // Crear archivo de log con fecha
        let log_file = format!(
            "{}/{}.{}.log",
            log_dir,
            app_name,
            Local::now().format("%Y-%m-%d")
        );
        
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_file)
            .unwrap_or_else(|_| {
                eprintln!("No se pudo abrir el archivo de log: {}", log_file);
                // Si no se puede abrir el archivo, usamos stderr
                std::process::exit(1);
            });
            
        Arc::new(Mutex::new(file))
    }).clone()
}

/// Inicializa el sistema de logging SOLO a consola (sin archivo)
/// Esto reduce el consumo de memoria significativamente
/// 
/// # Returns
/// * `Result<(), Box<dyn std::error::Error>>` - Ok si se inicializó correctamente
// pub fn init_logger_console_only() -> Result<(), Box<dyn std::error::Error>> {
//     // Filtro de niveles desde variable de entorno o predeterminado
//     let env_filter = EnvFilter::try_from_default_env()
//         .unwrap_or_else(|_| EnvFilter::new("info"));

//     // Solo consola, sin archivo
//     tracing_subscriber::registry()
//         .with(env_filter)
//         .with(fmt::layer().compact())
//         .init();

//     Ok(())
// }

/// Inicializa el sistema de logging con archivo (inicialización perezosa)
/// 
/// # Arguments
/// * `log_dir` - Directorio donde se guardarán los logs
/// * `app_name` - Nombre de la aplicación para los archivos de log
/// 
/// # Returns
/// * `Result<(), Box<dyn std::error::Error>>` - Ok si se inicializó correctamente
/// 
/// # Ejemplo
/// ```
/// use utils::logger::init_logger;
/// 
/// fn main() {
///     // Inicializar el logger (el archivo se creará con el primer mensaje)
///     init_logger("logs", "mi_app").expect("No se pudo inicializar el logger");
///     
///     // Los mensajes se escribirán tanto en consola como en archivo
///     tracing::info!("🚀 Iniciando aplicación");
/// }
/// ```
pub fn init_logger(log_dir: &'static str, app_name: &'static str) -> Result<(), Box<dyn std::error::Error>> {
    // Solo configuramos el logger una vez
    static INIT: OnceLock<()> = OnceLock::new();
    
    INIT.get_or_init(|| {
        // Configurar el filtro de niveles desde variable de entorno o predeterminado
        let env_filter = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("info"));
        
        // Configurar el escritor de archivos (inicialización perezosa)
        let file_writer = LockedMakeWriter::new();
        
        // Capa para archivo: SIN colores, formato compacto
        let file_layer = fmt::layer()
            .with_writer(file_writer)
            .with_ansi(false)
            .with_target(false)
            .compact();
        
        // Capa para consola: CON colores
        let stdout_layer = fmt::layer()
            .with_ansi(true)
            .compact();
        
        // Configurar el suscriptor
        tracing_subscriber::registry()
            .with(env_filter)
            .with(file_layer)
            .with(stdout_layer)
            .init();
            
        // Limpiar logs antiguos en segundo plano
        std::thread::spawn(move || {
            if let Err(e) = cleanup_old_logs(log_dir, 7) { // Conservar 7 días de logs
                eprintln!("Error limpiando logs antiguos: {}", e);
            }
        });
    });
    
    Ok(())
}

/// Limpia los archivos de log antiguos
/// 
/// # Arguments
/// * `log_dir` - Directorio de logs
/// * `max_age_days` - Número máximo de días de antigüedad para conservar los logs
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
