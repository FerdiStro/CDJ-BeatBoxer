#!/bin/bash

GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${BLUE}--- DJ Link Network Configurator ---${NC}"

# 1. Detect active network interfaces
# We search for device names associated with wifi or ethernet
WLAN_CON=$(nmcli -t -f NAME,DEVICE connection show --active | grep ":wlan" | cut -d: -f1 | head -n 1)
ETH_CON=$(nmcli -t -f NAME,DEVICE connection show --active | grep ":eth" | cut -d: -f1 | head -n 1)

# --- WLAN CONFIGURATION ---
if [ -z "$WLAN_CON" ]; then
    echo -e "${YELLOW}WARNING: No active WiFi connection found. SSH might break!${NC}"
else
    echo -e "${GREEN}WiFi found: '$WLAN_CON'${NC}"
    echo " -> Setting priority to HIGH (Metric 50) for Internet/SSH..."
    # Ensure WiFi is the preferred route for internet traffic
    sudo nmcli connection modify "$WLAN_CON" ipv4.route-metric 50
fi

echo "-------------------------------------------"

# --- ETHERNET CONFIGURATION (Static IP & Routes) ---
if [ -z "$ETH_CON" ]; then
    echo -e "${YELLOW}WARNING: No active Ethernet connection found.${NC}"
    # Try to find an inactive ethernet profile as fallback
    ETH_CON=$(nmcli -t -f NAME,TYPE connection show | grep ethernet | cut -d: -f1 | head -n 1)
fi

if [ ! -z "$ETH_CON" ]; then
    echo -e "${GREEN}Ethernet found: '$ETH_CON'${NC}"

    # 1. Set Metric to 2000 (Low priority for general internet)
    echo " -> Setting base metric to 2000..."
    sudo nmcli connection modify "$ETH_CON" ipv4.route-metric 2000

    # 2. Set Static IP (169.254.1.100)
    # This replaces: sudo ip addr add 169.254.1.100/16 dev eth0
    echo " -> Setting Static IP to 169.254.1.100/16..."
    sudo nmcli connection modify "$ETH_CON" ipv4.method manual ipv4.addresses 169.254.1.100/16

    # Remove gateway to prevent internet traffic via ETH
    sudo nmcli connection modify "$ETH_CON" ipv4.gateway ""

    # 3. Add Custom Routes (Persistently)
    # This replaces: sudo ip route add 169.254.0.0/16 ... metric 5
    echo " -> Adding persistent route for Link-Local (169.254.0.0/16)..."
    sudo nmcli connection modify "$ETH_CON" +ipv4.routes "169.254.0.0/16 0.0.0.0 5"

    # This replaces: sudo ip route add 224.0.0.0/4 ... metric 5 (Multicast)
    echo " -> Adding persistent route for Multicast (224.0.0.0/4)..."
    sudo nmcli connection modify "$ETH_CON" +ipv4.routes "224.0.0.0/4 0.0.0.0 5"

else
    echo -e "${YELLOW}ERROR: Could not find any Ethernet connection profile.${NC}"
fi

echo "-------------------------------------------"
echo -e "${BLUE}Applying changes...${NC}"

# Restart Ethernet to apply IP and Routes
if [ ! -z "$ETH_CON" ]; then
    echo "Restarting Ethernet..."
    sudo nmcli connection up "$ETH_CON"
fi

# Restart WiFi to ensure Metric 50 takes precedence
if [ ! -z "$WLAN_CON" ]; then
    echo "Restarting WiFi..."
    sudo nmcli connection up "$WLAN_CON"
fi

echo "-------------------------------------------"
echo -e "${GREEN}Done! Verifying Routes...${NC}"

# Display current routes to verify the metric 5 entries exist
ip route | grep "eth0"