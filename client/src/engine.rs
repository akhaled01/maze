use crate::renderer::Renderer;
use crate::resource_manager::ResourceManager;
use crate::audio::AudioManager;
use crate::player::Player;
use crate::weapon::Weapon;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::{GLProfile};
use std::time::{Instant, Duration};

pub struct Engine {
    sdl_context: sdl2::Sdl,
    video_subsystem: sdl2::VideoSubsystem,
    _gl_context: sdl2::video::GLContext,
    window: sdl2::video::Window,
    renderer: Renderer,
    resource_manager: ResourceManager,
    audio_manager: AudioManager,
    player: Player,
    weapon: Weapon,
    running: bool,
}

impl Engine {
    pub fn new() -> Self {
        // Init SDL2
        let sdl_context = sdl2::init().expect("Failed to initialize SDL2");
        let video_subsystem = sdl_context.video().expect("Couldn't get SDL video subsystem");

        // Setup OpenGL attributes
        {
            let gl_attr = video_subsystem.gl_attr();
            gl_attr.set_context_profile(GLProfile::Core);
            gl_attr.set_context_version(3, 3);
            gl_attr.set_double_buffer(true);
        }

        // Create window with OpenGL context
        let window = video_subsystem
            .window("Ranger Faggots", 1280, 720)
            .opengl()
            .position_centered()
            .build()
            .expect("Failed to create SDL window");

        let gl_context = window.gl_create_context().expect("Couldn't create GL context");
        window.gl_make_current(&gl_context).unwrap();

        // Enable VSync
        //video_subsystem.gl_set_swap_interval(sdl2::video::SwapInterval::VSync).unwrap();

        // Initialize OpenGL function pointers once
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const _);

        // Subsystem creation
        let renderer = Renderer::new();
        let resource_manager = ResourceManager::new();
        let audio_manager = AudioManager::new();
        let player = Player::new();
        let weapon = Weapon::new();

        Self {
            sdl_context,
            video_subsystem,
            _gl_context: gl_context,
            window,
            renderer,
            resource_manager,
            audio_manager,
            player,
            weapon,
            running: true,
        }
    }

    pub fn run(&mut self) {
        // Main game loop
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        let mut last_frame = Instant::now();

        while self.running {
            // --- Handle events ---
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => {
                        self.running = false;
                    }
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        self.running = false;
                    }
                    _ => {}
                }
            }

            // --- Calculate delta time ---
            let now = Instant::now();
            let delta = now.duration_since(last_frame);
            let dt = delta.as_secs_f32();
            last_frame = now;

            // --- Update game state ---
            self.update(dt);

            // --- Render ---
            self.render();

            // --- Sleep to control framerate (optional) ---
            // std::thread::sleep(Duration::from_millis(16));
        }
    }

    fn update(&mut self, dt: f32) {
        // Update all subsystems
        self.player.update();
        self.weapon.update();
        // TODO: Add more updates as systems are converted
    }

    fn render(&mut self) {
        self.renderer.render(&self.window);
        // TODO: Pass actual game state/render data to renderer as conversion proceeds
    }
}