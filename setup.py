from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="crdt_lww",
    version="0.1.0",
    packages=[],
    rust_extensions=[
        RustExtension("crdt_lww", path="crdt/Cargo.toml", binding=Binding.PyO3, features=["extension-module"]),
    ],
    include_package_data=True,
    zip_safe=False,
)
