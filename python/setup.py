"""
Setup script for StrataRouter Python package
"""

from setuptools import setup, find_packages
import os

# Read README
readme_path = os.path.join(os.path.dirname(__file__), '..', 'README.md')
with open(readme_path, 'r', encoding='utf-8') as f:
    long_description = f.read()

setup(
    name="stratarouter",
    version="0.1.0",
    packages=find_packages(),
    install_requires=[
        "numpy>=1.21.0",
        "pydantic>=2.0.0",
    ],
    extras_require={
        "dev": [
            "pytest>=7.0.0",
            "pytest-cov>=4.0.0",
            "black>=23.0.0",
            "ruff>=0.1.0",
            "mypy>=1.0.0",
        ],
        "openai": ["openai>=1.0.0"],
        "cohere": ["cohere>=4.0.0"],
        "huggingface": ["sentence-transformers>=2.2.0"],
        "all": [
            "openai>=1.0.0",
            "cohere>=4.0.0",
            "sentence-transformers>=2.2.0",
        ],
    },
    python_requires=">=3.8",
    author="StrataRouter Team",
    author_email="team@stratarouter.dev",
    description="High-performance semantic routing - 10x faster than semantic-router",
    long_description=long_description,
    long_description_content_type="text/markdown",
    url="https://github.com/stratarouter/stratarouter",
    classifiers=[
        "Development Status :: 4 - Beta",
        "Intended Audience :: Developers",
        "License :: OSI Approved :: MIT License",
        "Programming Language :: Python :: 3",
        "Programming Language :: Python :: 3.8",
        "Programming Language :: Python :: 3.9",
        "Programming Language :: Python :: 3.10",
        "Programming Language :: Python :: 3.11",
        "Programming Language :: Python :: 3.12",
        "Programming Language :: Rust",
        "Topic :: Scientific/Engineering :: Artificial Intelligence",
    ],
)
