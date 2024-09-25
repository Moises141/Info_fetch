InfoFetch

This Rust program retrieves and displays various system information for Linux-based distributions in a clean and user-friendly format. The tool is designed to work universally across different Linux distributions by using standard system commands and files, ensuring compatibility and portability.

Features

    OS Detection: Detects the current Linux distribution and architecture using /etc/os-release and environment variables.
    Hostname: Displays the current hostname of the machine.
    Kernel Version: Retrieves and displays the version of the Linux kernel.
    Uptime: Displays the system's uptime in hours and minutes, by reading from /proc/uptime.
    Installed Packages: Counts the number of installed packages on the system:
        RPM packages via the rpm command.
        Flatpak packages via the flatpak list command.
    Shell: Displays the current shell being used, retrieved from the SHELL environment variable.
    Resolution: Retrieves the screen resolution using the xdpyinfo command.
    Desktop Environment (DE): Displays the current desktop environment (GNOME, KDE, etc.) using the XDG_CURRENT_DESKTOP environment variable.
    Window Manager (WM): Displays the current window manager in use.
    CPU Information: Retrieves CPU details (model, number of cores, and speed) from /proc/cpuinfo.
    GPU Information: Detects the graphics processing unit (GPU) using the lspci command.
    Memory: Displays the current memory usage and total available memory using the sys_info crate.
    
    
Customization

You can easily customize the program to retrieve additional information by modifying or adding new functions. For example, you can add functionality to detect additional system components or specific Linux distribution features by integrating more commands or reading other system files.
Contribution

Feel free to fork this repository and submit pull requests with improvements or additional features.
