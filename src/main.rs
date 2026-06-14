use serde::{Serialize, Deserialize};
use colored::*;
use std::fs::File;
use std::io::{Write, Read, self};
use std::time::Instant;
use std::fmt;

// =========================================================================
// 1. MODELLER VE HATA YÖNETİMİ (Domain Layer)
// =========================================================================

#[derive(Debug, PartialEq)]
pub enum ExileFitError {
    EmptyExerciseName,
    InvalidWeight(f64),
    InvalidReps(u32),
    SessionNotFound,
    IOError(String),
}

impl fmt::Display for ExileFitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyExerciseName => write!(f, "Hata: Egzersiz adı boş bırakılamaz!"),
            Self::InvalidWeight(w) => write!(f, "Hata: Geçersiz ağırlık ({:.2} kg). Değer 0 veya daha büyük olmalıdır!", w),
            Self::InvalidReps(r) => write!(f, "Hata: Geçersiz tekrar sayısı ({}). En az 1 tekrar yapılmalıdır!", r),
            Self::SessionNotFound => write!(f, "Hata: Belirtilen antrenman oturumu sistemde bulunamadı!"),
            Self::IOError(err) => write!(f, "Dosya Sistemi Hatası: {}", err),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum MuscleGroup {
    Chest, // Göğüs
    Back,  // Sırt
    Arms,  // Kollar
    Legs,  // Bacaklar
    Core,  // Karın
}

impl fmt::Display for MuscleGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Chest => "Göğüs",
            Self::Back => "Sırt",
            Self::Arms => "Kollar",
            Self::Legs => "Bacaklar",
            Self::Core => "Karın",
        };
        write!(f, "{}", name)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Exercise {
    pub name: String,
    pub muscle_group: MuscleGroup,
    pub weight: f64,
    pub reps: u32,
    pub sets: u32,
}

impl Exercise {
    // Prototipi dayanıklı hale getiren girdi doğrulaması (Edge Case Control)
    pub fn new(name: &str, muscle_group: MuscleGroup, weight: f64, reps: u32, sets: u32) -> Result<Self, ExileFitError> {
        if name.trim().is_empty() {
            return Err(ExileFitError::EmptyExerciseName);
        }
        if weight < 0.0 {
            return Err(ExileFitError::InvalidWeight(weight));
        }
        if reps == 0 && sets > 0 {
            return Err(ExileFitError::InvalidReps(reps));
        }
        
        Ok(Exercise {
            name: name.trim().to_string(),
            muscle_group,
            weight,
            reps,
            sets,
        })
    }

    pub fn calculate_volume(&self) -> f64 {
        self.weight * (self.reps as f64) * (self.sets as f64)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WorkoutSession {
    pub id: u32,
    pub name: String,
    pub exercises: Vec<Exercise>,
}

impl WorkoutSession {
    pub fn new(id: u32, name: &str) -> Self {
        WorkoutSession {
            id,
            name: name.to_string(),
            exercises: Vec::new(),
        }
    }

    pub fn add_exercise(&mut self, exercise: Exercise) {
        self.exercises.push(exercise);
    }

    pub fn total_volume(&self) -> f64 {
        self.exercises.iter().map(|e| e.calculate_volume()).sum()
    }
}

// =========================================================================
// 2. SPRINT 4: VERİ YÖNETİMİ VE ANALİTİK MERKEZİ (Core Engine)
// =========================================================================

pub struct ExileFitSystem {
    pub sessions: Vec<WorkoutSession>,
    file_path: String,
}

impl ExileFitSystem {
    pub fn new(file_path: &str) -> Self {
        let mut system = ExileFitSystem {
            sessions: Vec::new(),
            file_path: file_path.to_string(),
        };
        // Program açılırken diskteki verileri otomatik yükle
        let _ = system.load_from_file();
        system
    }

    pub fn add_session(&mut self, session: WorkoutSession) -> Result<(), ExileFitError> {
        self.sessions.push(session);
        self.save_to_file()
    }

    // İkincil Özellik: ID ile güvenli arama
    pub fn get_session(&self, id: u32) -> Result<&WorkoutSession, ExileFitError> {
        self.sessions.iter()
            .find(|s| s.id == id)
            .ok_or(ExileFitError::SessionNotFound)
    }

    // Sprint 4 Yeniliği: JSON Dosyasına Kalıcı Kaydetme (Serialization)
    pub fn save_to_file(&self) -> Result<(), ExileFitError> {
        let json_data = serde_json::to_string_pretty(&self.sessions)
            .map_err(|e| ExileFitError::IOError(e.to_string()))?;
        let mut file = File::create(&self.file_path)
            .map_err(|e| ExileFitError::IOError(e.to_string()))?;
        file.write_all(json_data.as_bytes())
            .map_err(|e| ExileFitError::IOError(e.to_string()))?;
        Ok(())
    }

    // Sprint 4 Yeniliği: JSON Dosyasından Veri Okuma (Deserialization)
    pub fn load_from_file(&mut self) -> Result<(), ExileFitError> {
        if !std::path::Path::new(&self.file_path).exists() {
            return Ok(());
        }
        let mut file = File::open(&self.file_path)
            .map_err(|e| ExileFitError::IOError(e.to_string()))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|e| ExileFitError::IOError(e.to_string()))?;
        self.sessions = serde_json::from_str(&contents)
            .map_err(|e| ExileFitError::IOError(e.to_string()))?;
        Ok(())
    }

    // Sprint 4 Gelişmiş Analitik: Kişisel Rekor (PR) Hesaplama
    pub fn calculate_personal_record(&self, exercise_name: &str) -> Option<f64> {
        self.sessions.iter()
            .flat_map(|s| &s.exercises)
            .filter(|e| e.name.to_lowercase() == exercise_name.to_lowercase())
            .map(|e| e.weight)
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
    }

    // Performans Analiz Aracı: Büyük veri kümelerinde darboğaz takibi
    pub fn benchmark_analysis(&self) -> std::time::Duration {
        let start = Instant::now();
        // Yoğun hesaplama simülasyonu (Bütün antrenman hacimlerinin toplanması)
        let _: f64 = self.sessions.iter().map(|s| s.total_volume()).sum();
        start.elapsed()
    }
}

// =========================================================================
// 3. KULLANICI ARAYÜZÜ (Presentation Layer)
// =========================================================================

fn main() {
    println!("{}", "\n=======================================================".bold().cyan());
    println!("{}", "       EXILE.FIT - SPRINT 4 FINAL MVP RELEASE         ".bold().green());
    println!("{}", "=======================================================".bold().cyan());
    println!("Durum: {} | Veri Altyapısı: {}\n", "KARARLI".green().bold(), "JSON KALICI BELLEK".magenta().bold());

    // Sistemi dosya yolunu belirterek başlatıyoruz
    let mut system = ExileFitSystem::new("exile_fitness_data.json");

    // 1. Senaryo: Eğer veri tabanı boşsa mock veriler üretelim
    if system.sessions.is_empty() {
        println!("{}", "[Sistem] Yerel depolama boş. İlk çalıştırma verileri ekleniyor...".yellow());
        
        let mut session_1 = WorkoutSession::new(101, "Hipertrofi - İtiş Günü");
        
        if let Ok(e1) = Exercise::new("Bench Press", MuscleGroup::Chest, 95.0, 8, 4) {
            session_1.add_exercise(e1);
        }
        if let Ok(e2) = Exercise::new("Overhead Press", MuscleGroup::Arms, 55.0, 10, 3) {
            session_1.add_exercise(e2);
        }

        let mut session_2 = WorkoutSession::new(102, "Alt Vücut Güç Günü");
        if let Ok(e3) = Exercise::new("Squat", MuscleGroup::Legs, 145.0, 5, 3) {
            session_2.add_exercise(e3);
        }

        let _ = system.add_session(session_1);
        let _ = system.add_session(session_2);
        println!("{}", "✓ Başarılı: İlk veriler oluşturuldu ve 'exile_fitness_data.json' dosyasına kaydedildi.".green());
    } else {
        println!("{}", "✓ Başarılı: Kalıcı antrenman verileri JSON dosyasından başarıyla geri yüklendi.".green());
    }

    // 2. Senaryo: Gelişmiş Analitik ve PR Takip Gösterimi
    println!("\n{}", "--- 📊 GELİŞMİŞ ANALİTİK PANELİ ---".bold().white());
    let target_exercise = "Squat";
    match system.calculate_personal_record(target_exercise) {
        Some(pr) => {
            let log = format!("🔥 Kişisel Rekor (PR) [{}]: {} kg", target_exercise, pr);
            println!("{}", log.bold().yellow());
        }
        None => println!("Kayıtlı {} egzersizi bulunamadı.", target_exercise),
    }

    // 3. Senaryo: Dayanıklılık ve Kenar Durum Yönetimi (Zero-Panic Simülasyonu)
    println!("\n{}", "--- 🛡️ KENAR SENARYO & HATA YÖNETİMİ TESTİ ---".bold().white());
    
    // Hatalı Giriş Girişimi (Negatif Ağırlık)
    let invalid_input = Exercise::new("Deadlift", MuscleGroup::Back, -20.0, 5, 3);
    match invalid_input {
        Ok(_) => println!("Sistem hatalı veriyi kabul etti! (HATA)"),
        Err(e) => println!("✓ Güvenli Şekilde Engellendi -> {}", e.to_string().red()),
    }

    // Olmayan Oturum Sorgulama
    match system.get_session(999) {
        Ok(s) => println!("Oturum bulundu: {}", s.name),
        Err(e) => println!("✓ Güvenli Şekilde Yakalandı -> {}", e.to_string().red()),
    }

    // 4. Senaryo: Performans Darboğaz Raporlama
    let calc_duration = system.benchmark_analysis();
    println!("\n{}", "--- ⚡ PERFORMANS METRİKLERİ ---".bold().white());
    println!("Tüm veri seti hacim analiz süresi: {}", format!("{:?}", calc_duration).bold().cyan());

    println!("\n{}", "=======================================================".bold().cyan());
    println!("{}", "       EXILE.FIT - SPRINT 4 FINAL MVP RELEASE         ".bold().green());
    println!("{}", "=======================================================".bold().cyan());
}