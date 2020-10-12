from setuptools import setup, find_packages
import os.path

setup (
    name='TALA',
    version = "1",
    author='Ayden Drabek, Brennan Lamoreaux, Ethan Steidl, Ryan Shell, Timothy Adcock  ',
    author_email='ayden.drabek@mines.sdsmt.edu, brennan.lamoreaux@mines.sdsmt.edu, ethan.steidl@mines.sdsmt.edu, ryan.shell@mines.sdsmt.edu, timothy.adcock@mines.sdsmt.edu',
    packages = find_packages(),

    # package_dir={'mypkg': 'src/mypkg'},  # didnt use this.
    package_data = {
        # If any package contains *.txt or *.rst files, include them:
        "": ["*.png","*.ico"]
    },
    include_package_data=True,

#


)
