import setuptools

with open('README.md', 'r') as fh:
    long_description = fh.read()

setuptools.setup(
    name='pico_sdk',
    version='0.1.4',
    author='Meaty Solutions',
    author_email='info@meaty.io',
    description='High performance, gap-free streaming from any Pico Technology oscilloscope',
    long_description=long_description,
    long_description_content_type='text/markdown',
    url='https://github.com/meatysolutions/pico-sdk',
    package_data={'': ['artifacts/*', 'artifacts/*/*']},
    packages=setuptools.find_packages(),
    install_requires=['numpy'],
    classifiers=[
        'Programming Language :: Python :: 3',
        'License :: OSI Approved :: MIT License',
        'Operating System :: OS Independent',
    ],
    python_requires='>=3.6',
)
