//! AetherDCC Studio v1.0.
//! Quaternary CPU contention, Intel UHD 0x9B41 ternary digit inference,
//! GhostVault, and the 64-node Ising paradox solver.

mod abi;
mod bitnet;
mod cpu_alu;
mod gpu;
mod ising;
mod machine;
mod product;
mod report;
mod scenarios;
mod sentinel;

pub use bitnet::{argmax, corrupt, glyph_f32, ratio, reference, weight_census, GLYPHS};
pub use cpu_alu::Alu;
pub use gpu::{BitnetSample, GpuEngine, GPU_ADD, GPU_XOR};
pub use product::{digit_qa, product_benchmark, DigitQa};
pub use report::{print_scorecard, ScenarioReport};
pub use scenarios::{
    run_all, stress_adversarial_alu_fuzzer, stress_concurrent_cpu_gpu_collision, stress_endurance_marathon,
    stress_hostile_os_storm, stress_pathological_paradox_and_graphs, stress_vision_noise_cliff_and_zero_fix,
};

#[cfg(test)]
mod tests {
    use super::print_scorecard;

    #[test]
    fn stress_hostile_os_storm() {
        let row = super::stress_hostile_os_storm();
        print_scorecard(std::slice::from_ref(&row));
        assert!(row.passed, "hostile storm recorded {} errors", row.errors);
    }

    #[test]
    fn stress_endurance_marathon() {
        let row = super::stress_endurance_marathon();
        print_scorecard(std::slice::from_ref(&row));
        assert!(row.passed, "endurance recorded {} errors", row.errors);
    }

    #[test]
    fn stress_concurrent_cpu_gpu_collision() {
        let row = super::stress_concurrent_cpu_gpu_collision();
        print_scorecard(std::slice::from_ref(&row));
        assert!(row.passed, "collision recorded {} errors", row.errors);
    }

    #[test]
    fn stress_adversarial_alu_fuzzer() {
        let row = super::stress_adversarial_alu_fuzzer();
        print_scorecard(std::slice::from_ref(&row));
        assert!(row.passed, "fuzzer recorded {} errors", row.errors);
    }

    #[test]
    fn stress_vision_noise_cliff_and_zero_fix() {
        let row = super::stress_vision_noise_cliff_and_zero_fix();
        print_scorecard(std::slice::from_ref(&row));
        assert!(row.passed, "vision cliff recorded {} errors", row.errors);
    }

    #[test]
    fn stress_pathological_paradox_and_graphs() {
        let row = super::stress_pathological_paradox_and_graphs();
        print_scorecard(std::slice::from_ref(&row));
        assert!(row.passed, "paradox/graphs recorded {} errors", row.errors);
    }

    #[test]
    fn product_bitnet_digit10() {
        let mut gpu = super::GpuEngine::new();
        let qa = super::digit_qa(&mut gpu);
        assert_eq!(qa.blank_max, 0, "empty canvas was not DC-centered");
        assert_eq!(qa.clean, 10, "clean digits");
        assert_eq!(qa.band_misses, 0, "noise band 0..=12 flips");
        assert_eq!(qa.mismatches, 0, "GPU diverged from the host reference");
    }
}
