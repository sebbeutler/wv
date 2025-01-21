import socket
import ipaddress
import threading

# Function to scan a single IP for open ports
def scan_ports(ip):
    print(f"Scanning IP: {ip}")
    open_ports = []
    
    for port in range(1, 1025):  # Scans ports 1 to 1024
        try:
            sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
            sock.settimeout(0.5)
            result = sock.connect_ex((ip, port))
            if result == 0:
                open_ports.append(port)
            sock.close()
        except Exception as e:
            pass

    if open_ports:
        print(f"Open ports on {ip}: {open_ports}")
    else:
        print(f"No open ports found on {ip}")

# Function to discover devices on the network
def discover_devices(network):
    print("Discovering devices...")
    devices = []

    for ip in ipaddress.IPv4Network(network, strict=False):
        print(f"Scanning: {ip}", end="\n")
        try:
            sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
            sock.settimeout(0.5)
            result = sock.connect_ex((str(ip), 135))  # Common port to check if a device is active
            if result == 0:
                print(f"Found: {ip}")
                devices.append(str(ip))
            sock.close()
        except Exception as e:
            pass

    if devices:
        print("Devices found on the network:")
        for device in devices:
            print(device)
    else:
        print("No devices found on the network.")

# Main function
def main():
    print("Network Scanner Tool")
    print("1. Discover devices in a network")
    print("2. Scan a device for open ports")

    choice = input("Enter your choice (1/2): ")

    if choice == '1':
        network = input("Enter network range (e.g., 192.168.1.0/24): ")
        discover_devices(network)
    elif choice == '2':
        ip = input("Enter the IP address to scan: ")
        scan_ports(ip)
    else:
        print("Invalid choice. Exiting.")

if __name__ == "__main__":
    main()