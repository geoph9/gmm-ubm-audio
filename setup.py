import sys
from setuptools import setup

try:
    from setuptools_rust import RustExtension
except ImportError:
    import subprocess
    errno = subprocess.call(
        [sys.executable, '-m', 'pip', 'install', 'setuptools-rust'])
    if errno:
        print("Please install setuptools-rust package")
        raise SystemExit(errno)
    else:
        from setuptools_rust import RustExtension

setup_requires = ['setuptools-rust>=0.11.6']
install_requires = ['librosa==0.8.0', 'numpy==1.20.1']

setup(name='gmmubm',
      version='0.1.0',
      classifiers=[
          'License :: OSI Approved :: MIT License',
          'Development Status :: 3 - Alpha',
          'Intended Audience :: Developers',
          'Programming Language :: Python',
          'Programming Language :: Rust',
          'Operating System :: POSIX',
      ],
      rust_extensions=[
          RustExtension('gmmubm.gmmubm', 'Cargo.toml', debug=False)],
      packages=['gmmubm'],
      include_package_data=True,
      zip_safe=False
)
