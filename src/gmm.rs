// This file will fit a GMM model using the EM algorithm.
// This GMM is going to be the universal model (UBM) which is trained on the
// whole dataset regardless of the labels.
// Then, we perform MAP adaptation in order to get the specific GMMs for each
// label (check map.rs).

