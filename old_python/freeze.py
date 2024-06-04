import sys
from cx_Freeze import setup, Executable

# Dependencies are automatically detected, but it might need fine-tuning.
build_exe_options = {
    "packages": ["os", "requests", "tkinter"],  # Add necessary packages
    "excludes": [],
    "include_files": [],  # Add any additional files needed
}

base = None
if sys.platform == "win32":
    base = "Win32GUI"  # Use "Win32GUI" for a GUI application

setup(
    name="TestResultChecker",
    version="1.0",
    description="Test Result Checker Application",
    options={"build_exe": build_exe_options},
    executables=[Executable("main.py", base=base, target_name="TestResultChecker.exe")],
)
