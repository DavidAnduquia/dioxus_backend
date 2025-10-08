use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::sync::{Arc, Mutex};

use chrono::Local;
use tracing_subscriber::{fmt::writer::MakeWriter, layer::SubscriberExt, EnvFilter, fmt, util::SubscriberInitExt};

#[derive(Clone)]
struct LockedMakeWriter {
    file: Arc<Mutex<File>>,
}

struct LockedWriter {
    file: Arc<Mutex<File>>,
}

impl Write for LockedWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut file = self.file.lock().unwrap();
        file.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        let mut file = self.file.lock().unwrap();
        file.flush()
    }
}

impl<'a> MakeWriter<'a> for LockedMakeWriter {
    type Writer = LockedWriter;

    fn make_writer(&'a self) -> Self::Writer {
        LockedWriter {
            file: Arc::clone(&self.file),
        }
    }
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

/// Inicializa el sistema de logging con archivo (consume más memoria)
/// 
/// # Arguments
/// * `log_dir` - Directorio donde se guardarán los logs
/// * `app_name` - Nombre de la aplicación para los archivos de log
/// 
/// # Returns
/// * `Result<(), Box<dyn std::error::Error>>` - Ok si se inicializó correctamente
pub fn init_logger(log_dir: &str, app_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Crear directorio de logs si no existe
    std::fs::create_dir_all(log_dir)?;

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
        .open(log_file)?;
    
    let file = Arc::new(Mutex::new(file));
    let file_writer = LockedMakeWriter {
        file: Arc::clone(&file),
    };

    // Filtro de niveles desde variable de entorno o predeterminado
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

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

    Ok(())
}

/// Limpia los archivos de log antiguos
/// 
/// # Arguments
/// * `log_dir` - Directorio de logs
/// * `max_age_days` - Número máximo de días de antigüedad para conservar los logs
pub fn cleanup_old_logs(log_dir: &str, max_age_days: u64) -> std::io::Result<()> {
    use std::time::{SystemTime, Duration};

    let now = SystemTime::now();
    let max_age = Duration::from_secs(max_age_days * 24 * 3600);

    for entry in fs::read_dir(log_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            if let Ok(metadata) = fs::metadata(&path) {
                if let Ok(modified) = metadata.modified() {
                    if let Ok(elapsed) = now.duration_since(modified) {
                        if elapsed > max_age {
                            tracing::info!("🗑️  Eliminando log antiguo: {:?}", path);
                            fs::remove_file(path)?;
                        }
                    }
                }
            }
        }
    }
    
    Ok(())
}
