# Install Libxml2 with vcpkg and set environment

# Determine architecture
# https://blog.netnerds.net/2024/07/arm64-powershell-pwsh/
$osPlatform = [System.Runtime.InteropServices.RuntimeInformation]::OSDescription
$architecture = [System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture

echo "Platform: $osPlatform"
echo "Architecture: $architecture"

if (-Not ($osPlatform.Contains("Windows"))) {
  echo "Not windows"
  exit 1;
}

$directory = "$architecture-windows".ToLower()

echo "CMAKE_TOOLCHAIN_FILE=C:/vcpkg/scripts/buildsystems/vcpkg.cmake" >> $env:GITHUB_ENV
echo "PC_LIBXML_INCLUDE_DIRS=C:/vcpkg/$directory/include/libxml2" >> $env:GITHUB_ENV
Get-ChildItem env:
