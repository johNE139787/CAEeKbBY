use rocket::get;
use rocket::serde::json::Json;
use rocket::State;
use std::collections::HashMap;
use std::sync::Mutex;

// Define a struct to represent a Sound
struct Sound {
    id: String,
    name: String,
    // Additional fields can be added here to represent sound properties
}

// Define a manager to control sounds
struct SoundManager {
    sounds: Mutex<HashMap<String, Sound>>,
}

#[macro_use] extern crate rocket;

// Implement methods for SoundManager
impl SoundManager {
    // Create a new SoundManager
    pub fn new() -> Self {
        SoundManager {
            sounds: Mutex::new(HashMap::new()),
        }
    }

    // Play a sound by its ID
    pub fn play(&self, sound_id: &str) -> Result<(), String> {
        let mut sounds = self.sounds.lock().unwrap();
        match sounds.get_mut(sound_id) {
            Some(sound) => {
                // Simulate playing the sound
                println!("Playing sound: {}", sound.name);
                Ok(())
            },
            None => Err(format!("Sound with ID {} not found", sound_id)),
        }
    }

    // Pause a sound by its ID
    pub fn pause(&self, sound_id: &str) -> Result<(), String> {
        let mut sounds = self.sounds.lock().unwrap();
        match sounds.get_mut(sound_id) {
            Some(sound) => {
                // Simulate pausing the sound
                println!("Pausing sound: {}", sound.name);
                Ok(())
            },
            None => Err(format!("Sound with ID {} not found", sound_id)),
        }
    }

    // Stop a sound by its ID
    pub fn stop(&self, sound_id: &str) -> Result<(), String> {
        let mut sounds = self.sounds.lock().unwrap();
        match sounds.get_mut(sound_id) {
            Some(sound) => {
                // Simulate stopping the sound
                println!("Stopping sound: {}", sound.name);
                Ok(())
            },
            None => Err(format!("Sound with ID {} not found", sound_id)),
        }
    }

    // Add a new sound to the manager
    pub fn add_sound(&self, sound: Sound) {
        let mut sounds = self.sounds.lock().unwrap();
        sounds.insert(sound.id.clone(), sound);
    }
}

// Define a state for the SoundManager
#[rocket::get("/play/<sound_id>