use crate::back::lto::{LtoModuleCodegen, SerializedModule, ThinModule};
use crate::back::write::{CodegenContext, ModuleConfig};
use crate::{CompiledModule, ModuleCodegen};

use rustc::dep_graph::WorkProduct;
use rustc::util::time_graph::Timeline;
use rustc_errors::{FatalError, Handler};

pub trait WriteBackendMethods: 'static + Sized + Clone {
    type Module: Send + Sync;
    type TargetMachine;
    type ModuleBuffer: ModuleBufferMethods;
    type Context: ?Sized;
    type ThinData: Send + Sync;
    type ThinBuffer: ThinBufferMethods;

    /// Performs fat LTO by merging all modules into a single one and returning it
    /// for further optimization.
    fn run_fat_lto(
        cgcx: &CodegenContext<Self>,
        modules: Vec<ModuleCodegen<Self::Module>>,
        timeline: &mut Timeline,
    ) -> Result<LtoModuleCodegen<Self>, FatalError>;
    /// Performs thin LTO by performing necessary global analysis and returning two
    /// lists, one of the modules that need optimization and another for modules that
    /// can simply be copied over from the incr. comp. cache.
    fn run_thin_lto(
        cgcx: &CodegenContext<Self>,
        modules: Vec<(String, Self::ThinBuffer)>,
        cached_modules: Vec<(SerializedModule<Self::ModuleBuffer>, WorkProduct)>,
        timeline: &mut Timeline,
    ) -> Result<(Vec<LtoModuleCodegen<Self>>, Vec<WorkProduct>), FatalError>;
    fn print_pass_timings(&self);
    unsafe fn optimize(
        cgcx: &CodegenContext<Self>,
        diag_handler: &Handler,
        module: &ModuleCodegen<Self::Module>,
        config: &ModuleConfig,
        timeline: &mut Timeline,
    ) -> Result<(), FatalError>;
    unsafe fn optimize_thin(
        cgcx: &CodegenContext<Self>,
        thin: &mut ThinModule<Self>,
        timeline: &mut Timeline,
    ) -> Result<ModuleCodegen<Self::Module>, FatalError>;
    unsafe fn codegen(
        cgcx: &CodegenContext<Self>,
        diag_handler: &Handler,
        module: ModuleCodegen<Self::Module>,
        config: &ModuleConfig,
        timeline: &mut Timeline,
    ) -> Result<CompiledModule, FatalError>;
    fn prepare_thin(
        cgcx: &CodegenContext<Self>,
        module: ModuleCodegen<Self::Module>
    ) -> (String, Self::ThinBuffer);
    fn run_lto_pass_manager(
        cgcx: &CodegenContext<Self>,
        llmod: &ModuleCodegen<Self::Module>,
        config: &ModuleConfig,
        thin: bool,
    );
}

pub trait ThinBufferMethods: Send + Sync {
    fn data(&self) -> &[u8];
}

pub trait ModuleBufferMethods: Send + Sync {
    fn data(&self) -> &[u8];
}
