use sabbir_hossain as sabbir;

fn main() {
    //
    #[cfg(target_arch = "wasm32")]
    {
       sabbir::run_web().unwrap();
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        sabbir::run().unwrap();
    }
}
