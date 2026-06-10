use std::io::{self, Write};

#[derive(Debug)]
enum MuscleGroup {
    Chest,
    Back,
    Arms,
    Legs,
    Shoulders,
    Core,
}

#[derive(Debug)]
struct Exercise {
    name: String,
    muscle_group: MuscleGroup,
    sets: u32,
    reps: u32,
    weight: f32,
}

impl Exercise {
    fn calculate_volume(&self) -> f32 {
        (self.sets * self.reps) as f32 * self.weight
    }
}

#[derive(Debug)]
struct WorkoutSession {
    exercises: Vec<Exercise>,
}

impl WorkoutSession {
    fn new() -> Self {
        WorkoutSession {
            exercises: Vec::new(),
        }
    }

    fn add_exercise(&mut self, exercise: Exercise) {
        self.exercises.push(exercise);
    }

    fn total_volume(&self) -> f32 {
        self.exercises.iter().map(|e| e.calculate_volume()).sum()
    }

    fn display_summary(&self) {
        println!("\n--- 🏋️ Antrenman Özeti 🏋️ ---");
        for ex in &self.exercises {
            println!(
                "Egzersiz: {} ({:?}) - {} Set x {} Tekrar x {} kg -> Hacim: {} kg",
                ex.name,
                ex.muscle_group,
                ex.sets,
                ex.reps,
                ex.weight,
                ex.calculate_volume()
            );
        }
        println!("Toplam Antrenman Hacmi: {} kg", self.total_volume());
        println!("------------------------------\n");
    }
}

// Kullanıcıdan metin almak için yardımcı fonksiyon
fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Girdi okunamadı");
    input.trim().to_string()
}

// Kullanıcıdan sayı almak ve hata yönetimi yapmak için fonksiyon (Sprint 3 Özelliği)
fn get_number_input<T: std::str::FromStr>(prompt: &str) -> T {
    loop {
        let input = get_input(prompt);
        match input.parse::<T>() {
            Ok(num) => return num,
            Err(_) => println!("⚠️ Hata: Lütfen geçerli bir sayı girin!"),
        }
    }
}

fn main() {
    println!("🚀 Exile.Fit - Sprint 3 Sürümüne Hoş Geldiniz!");
    let mut session = WorkoutSession::new();

    loop {
        println!("\n[1] Yeni Egzersiz Ekle");
        println!("[2] Antrenmanı Bitir ve Özeti Gör");
        let choice = get_input("Seçiminiz (1 veya 2): ");

        match choice.as_str() {
            "1" => {
                let name = get_input("Egzersiz Adı (örn. Bench Press): ");
                
                println!("Kas Grupları: 1-Göğüs, 2-Sırt, 3-Kollar, 4-Bacak, 5-Omuz, 6-Core");
                let muscle_group = loop {
                    let mg_input = get_input("Kas Grubu (1-6): ");
                    match mg_input.as_str() {
                        "1" => break MuscleGroup::Chest,
                        "2" => break MuscleGroup::Back,
                        "3" => break MuscleGroup::Arms,
                        "4" => break MuscleGroup::Legs,
                        "5" => break MuscleGroup::Shoulders,
                        "6" => break MuscleGroup::Core,
                        _ => println!("⚠️ Hata: Lütfen 1 ile 6 arasında bir sayı girin!"),
                    }
                };

                let sets: u32 = get_number_input("Set Sayısı: ");
                let reps: u32 = get_number_input("Tekrar Sayısı: ");
                let weight: f32 = get_number_input("Ağırlık (kg): ");

                let exercise = Exercise {
                    name,
                    muscle_group,
                    sets,
                    reps,
                    weight,
                };

                session.add_exercise(exercise);
                println!("✅ Egzersiz başarıyla eklendi!");
            }
            "2" => {
                session.display_summary();
                println!("💪 Antrenman kaydedildi. Çıkış yapılıyor...");
                break;
            }
            _ => println!("⚠️ Hata: Lütfen sadece 1 veya 2'yi seçin!"),
        }
    }
}