from setuptools import setup, find_packages
import os.path

setup (
    name='TALA',
    version = "1",
    packages = find_packages(),

    # package_dir={'mypkg': 'src/mypkg'},  # didnt use this.
    package_data = {
        # If any package contains *.txt or *.rst files, include them:
        "": ["*.png","*.ico"]
    },
    include_package_data=True,

#


)
