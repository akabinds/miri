use rustc_middle::mir;

use log::trace;

use crate::*;

#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum Dlsym {
}

impl Dlsym {
    // Returns an error for unsupported symbols, and None if this symbol
    // should become a NULL pointer (pretend it does not exist).
    pub fn from_str<'tcx>(name: &str) -> InterpResult<'tcx, Option<Dlsym>> {
        Ok(match name {
            _ => throw_unsup_format!("unsupported freebsd dlsym: {}", name),
        })
    }
}

impl<'mir, 'tcx: 'mir> EvalContextExt<'mir, 'tcx> for crate::MiriEvalContext<'mir, 'tcx> {}
pub trait EvalContextExt<'mir, 'tcx: 'mir>: crate::MiriEvalContextExt<'mir, 'tcx> {
    fn call_dlsym(
        &mut self,
        dlsym: Dlsym,
        args: &[OpTy<'tcx, Tag>],
        dest: &PlaceTy<'tcx, Tag>,
        ret: Option<mir::BasicBlock>,
    ) -> InterpResult<'tcx> {
        let this = self.eval_context_mut();
        let ret = ret.expect("we don't support any diverging dlsym");
        assert!(this.tcx.sess.target.os == "freebsd");

        match dlsym {
            Dlsym::getentropy => {
                let [ptr, len] = check_arg_count(args)?;
                let ptr = this.read_pointer(ptr)?;
                let len = this.read_scalar(len)?.to_machine_usize(this)?;
                this.gen_random(ptr, len)?;
                this.write_null(dest)?;
            }
        }

        trace!("{:?}", this.dump_place(**dest));
        this.go_to_block(ret);
        Ok(())
    }
}
