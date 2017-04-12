#![no_std]
#![no_main]
#![feature(collections)]
#![feature(core_intrinsics)]

extern crate stm32f7_discovery as stm32f7;
#[macro_use]
extern crate collections;
// initialization routines for .data and .bss
extern crate r0;

mod hardware;
mod visuals;
mod audio;
mod transformation;

use collections::boxed::Box;
use visuals::constants::*;
use visuals::default_visualizer::DefaultVisualizer;
use visuals::direct_mic_visualizer::DirectMicVisualizer;
use visuals::energy_visualizer::EnergyVisualizer;
use visuals::direct_mic_batch_vz::DirectMicBatchVisualizer;
use visuals::sliding_sound_wave_vz::SlidingSoundVisualizer;
use visuals::sliding_sound_wave_points_vz::SlidingSoundPointsVisualizer;
use visuals::spectrum_visualizer::SpectrumVisualizer;
use visuals::spectrum_visualizer2::SpectrumVisualizer2;

use visuals::Visualizer;

use stm32f7::lcd;
use stm32f7::touch;

#[inline(never)]
fn main() -> ! {
    let mut stm = hardware::STM::init();
    stm.lcd.clear_screen();

    let mut visualizers = vec![];
    /*
    The defult VZ draws something
     ========================
    */
    let default_viz: Box<Visualizer> =  DefaultVisualizer::new(
                          0xFFFF,
                          0xFC00);
    visualizers.push(default_viz);
    /*
    DirectMicVZ shows the soundwave from one mic. Draws one sample at at time from left to right, followed by clearscreen
    ========================
    */
    let direct_mic_viz: Box<Visualizer> = DirectMicVisualizer::new(2);
    visualizers.push(direct_mic_viz);
    /*
    DirectMicBatchVZ shows the soundwave from one mic like DirectSoundMic, but receives a batch of samples
    ========================
    */
    let direct_mic_batch_viz: Box<Visualizer> = DirectMicBatchVisualizer::new(2);
    //visualizers.push(direct_mic_batch_viz);
    /*
    SlidingSoundVZ shows the soundwave from one mic by sliding the shown area to the right upon receiving a new sample
    draws bars
    ========================
    */
    let sliding_viz: Box<Visualizer> = SlidingSoundVisualizer::new(2);
    visualizers.push(sliding_viz);
    /*
    SlidingSoundPointsVZ shows the soundwave from one mic by sliding the shown area to the right upon receiving a new sample
    draws points
    ========================
    */
    let sliding_points_viz: Box<Visualizer> = SlidingSoundPointsVisualizer::new( 2, RED, BLACK);
    visualizers.push(sliding_points_viz);
    /*
    EnergyVZ shows a circle indicating the energy of the given samples (experimental)
    ========================
    */
    let energy_viz: Box<Visualizer> = EnergyVisualizer::new();
    //visualizers.push(energy_viz);
     /*
    SpectrumVZ shows the spectrum of the mic input
    ========================
    */
    let spectrum_viz2: Box<Visualizer> = SpectrumVisualizer2::new(2, GREEN, RED, BLACK);
    visualizers.push(spectrum_viz2);
    /*
    SpectrumVZ shows the result of the frequency analysis
    ========================
    */
    let spectrum_viz: Box<Visualizer> = Box::new(SpectrumVisualizer::new());
    //visualizers.push(spectrum_viz);

    let mut current_visualizer = 0;

    stm.lcd.set_background_color(lcd::Color::rgb(0, 0, 0));
    loop {
        visualizers[current_visualizer].draw(&mut stm);
        // poll for new touch data
        if touch::touches(&mut stm.i2c_3).unwrap().len() > 0 {
            current_visualizer = (current_visualizer +1) % visualizers.len();
            stm.lcd.clear_screen();
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn reset() -> ! {
    extern "C" {
        static __DATA_LOAD: u32;
        static __DATA_END: u32;
        static mut __DATA_START: u32;
        static mut __BSS_START: u32;
        static mut __BSS_END: u32;
    }
    let data_load = &__DATA_LOAD;
    let data_start = &mut __DATA_START;
    let data_end = &__DATA_END;
    let bss_start = &mut __BSS_START;
    let bss_end = &__BSS_END;

    // initializes the .data section
    //(copy the data segment initializers from flash to RAM)
    r0::init_data(data_start, data_end, data_load);
    // zeroes the .bss section
    r0::zero_bss(bss_start, bss_end);

    stm32f7::heap::init();

    // enable floating point unit
    let scb = stm32f7::cortex_m::peripheral::scb_mut();
    scb.cpacr.modify(|v| v | 0b1111 << 20);

    main();
}
