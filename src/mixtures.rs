use ndarray::{Array1, Array2};

enum MixType {
    GMM,
    UBM
}

enum CovarType {
    DIAG,
    FULL
}

pub struct Mixture {
    mix_type: MixType,
    covar_type: CovarType,
    dim: Option<usize>,
    ncentres: Option<u8>,
    priors: Option<Array1<f64>>,
    covars: Option<Array2<f64>>,
    centres: Option<Array1<f64>>,
    nwts: Option<Array1<f64>>
}

impl Mixture {
    fn new(mix_type: MixType,
           covar_type: CovarType,
           dim: Option<usize>,
           ncentres: Option<u8>,
           priors: Option<Array1<f64>>,
           covars: Option<Array2<f64>>,
           centres: Option<Array1<f64>>,
           nwts: Option<Array1<f64>>) -> Self {
        Self {
            mix_type, covar_type, dim, ncentres, priors, covars, centres, nwts
        }
    }

    pub(crate) fn priors(& self) -> Array1<f64> {
        self.priors.expect("Could not unwrap priors...")
    }

    pub(crate) fn ncentres(& self) -> u8 {
        self.ncentres.expect("Could not unwrap ncentres...")
    }
}

struct GMM {

}