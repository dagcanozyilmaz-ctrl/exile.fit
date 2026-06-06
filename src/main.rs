use std::fmt;

// --- 1. ENUMLAR ---
#[derive(Debug, Clone)]
enum MuscleGroup {
    Chest, Back, Arms, Legs, Core,
}

impl fmt::Display for MuscleGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// --- 2. STRUCTLAR ---
#[derive(Debug, Clone)]
struct Exercise {
    name: String,
    target_muscle: MuscleGroup,
    sets: u32,
    reps: u32,
    weight_kg: f32,
}

impl Exercise {
    fn new(name: &str, target_muscle: MuscleGroup, sets: u32, reps: u32, weight_kg: f32) -> Self {
        Exercise {
            name: name.to_string(),
            target_muscle,
            sets,
            reps,
            weight_kg,
        }
    }
    fn calculate_volume(&self) -> f32 {
        (self.sets * self.reps) as f32 * self.weight_kg
    }
}

#[derive(Debug)]
struct WorkoutSession {
    session_name: String,
    exercises: Vec<Exercise>, 
}

impl WorkoutSession {
    fn new(name: &str) -> Self {
        WorkoutSession {
            session_name: name.to_string(),
            exercises: Vec::new(),
        }
    }
    fn add_exercise(&mut self, exercise: Exercise) {
        self.exercises.push(exercise);
    }
    fn display_summary(&self) {
        println!("\n=========================================");
        println!("🏋️‍♂️ ANTRENMAN ÖZETİ: {}", self.session_name);
        println!("=========================================");
        let mut total_session_volume = 0.0;
        for (index, exercise) in self.exercises.iter().enumerate() {
            let volume = exercise.calculate_volume();
            total_session_volume += volume;
            println!("{}. {} ({})", index + 1, exercise.name, exercise.target_muscle);
            println!("   ↳ {} Set x {} Tekrar | Ağırlık: {} kg", exercise.sets, exercise.reps, exercise.weight_kg);
            println!("   ↳ Bu Egzersizin Toplam Hacmi: {} kg\n", volume);
        }
        println!("-----------------------------------------");
        println!("📊 Toplam Antrenman Hacmi: {} kg", total_session_volume);
        println!("=========================================\n");
    }
}

// --- 3. MAIN ---
fn main() {
    println!("--- Exile.Fit Fitness Takip Sistemi (2. Sprint) ---");
    let mut bugunku_antrenman = WorkoutSession::new("Anlık Antrenman Seansı");
    
    // Örnek egzersizler
    let exercise_1 = Exercise::new("Chest Press", MuscleGroup::Chest, 4, 10, 60.0);
    let exercise_2 = Exercise::new("Lat Pulldown", MuscleGroup::Back, 4, 12, 55.0);
    let exercise_3 = Exercise::new("Bicep Curl", MuscleGroup::Arms, 3, 12, 12.5);

    bugunku_antrenman.add_exercise(exercise_1);
    bugunku_antrenman.add_exercise(exercise_2);
    bugunku_antrenman.add_exercise(exercise_3);

    bugunku_antrenman.display_summary();
}