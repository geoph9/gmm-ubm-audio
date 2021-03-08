import argparse
# import glob
import os
import random
import warnings

import numpy as np
import librosa


def sample():
    return 1


class Converter:
    def __init__(self, wav_directory, out_directory, out_directory_test=None,
                 sr=8000, n_mfccs=20, split_percentage=0.8):
        assert 0 < split_percentage <= 1, "Split percentage must be in the range (0, 1]."
        self.wav_dir = wav_directory
        self.out_dir_train = out_directory
        self.out_dir_test = out_directory_test
        self.sr = sr
        self.n_mfccs = n_mfccs
        # Splitting into training and test set
        self.split_percentage = split_percentage

    # TODO: generalize to allow more feature extraction methods
    def extract(self, **kwargs):
        # Avoid using this if inside the for loop
        if self.out_dir_test is None:
            sampler = sample
        else:
            warnings.warn("In order to split into training and test sets, we will use a {}% split. "
                          "Note that the split will not be 100% on point since the decision on whether an "
                          "audio file belongs to the train or test set is done randomly.".
                          format(self.split_percentage * 100))
            sampler = random.random
        # for file in glob.glob(os.path.join(self.wav_dir, "*.wav")):
        for root, dirs, files in os.walk(self.wav_dir):
            for file in files:
                if file.endswith(".wav"):
                    sig, _ = librosa.load(file, sr=self.sr)
                    feas = librosa.feature.mfcc(y=sig, sr=self.sr, n_mfcc=self.n_mfccs, **kwargs)
                    # If split_percentage=0.8 then with 80% chance we will save the wav file in the training dir
                    # and with 20% chance we will save it in the test dir.
                    if sampler() <= self.split_percentage:
                        np.save(os.path.join(self.out_dir_train, os.path.basename(file).replace(".wav", ".npy")), feas)
                    else:
                        np.save(os.path.join(self.out_dir_test, os.path.basename(file).replace(".wav", ".npy")), feas)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--wav-directory", "--in-dir", "-i", dest="wav_directory",
                        help="Path to a directory containing .wav files. Must exist.")
    parser.add_argument("--out-directory", "--out-dir", "-o", dest="out_directory",
                        help="Path to where the output .npy files containing the mfcc features will be saved")
    parser.add_argument("--sample-rate", "--sr", "-r", dest='sr', choices=[8000, 16000, 44100], default=8000,
                        help="Sample rate that you want your audios to have")
    parser.add_argument("--n-mfccs", "-m", dest='n_mfccs', type=int, default=20,
                        help="Number of MFCCs to keep during the extraction.")
    args = parser.parse_args()
    if not os.path.isdir(args.wav_directory):
        raise argparse.ArgumentTypeError("Invalid wav directory (does not exist): {}.".format(args.wav_directory))
    conv = Converter(args.wav_directory, args.out_directory, args.sr, args.n_mfccs)
    conv.extract(n_mfcc=args.n_mfccs)


if __name__ == "__main__":
    main()
