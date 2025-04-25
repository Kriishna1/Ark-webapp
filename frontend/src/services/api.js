// src/services/api.js
const API_BASE_URL = 'http://localhost:8080';

export const walletService = {
  // Create a new wallet
  createWallet: async () => {
    const response = await fetch(`${API_BASE_URL}/create_wallet`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      }
    });
    
    if (!response.ok) {
      throw new Error(`Failed to create wallet: ${response.statusText}`);
    }
    
    return await response.json();
  },
  
  // Get wallet addresses
  getAddresses: async (walletId) => {
    const response = await fetch(`${API_BASE_URL}/get_address/${walletId}`, {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json'
      }
    });
    
    if (!response.ok) {
      throw new Error(`Failed to get addresses: ${response.statusText}`);
    }
    
    return await response.json();
  },
  
  // Get wallet balance
  getBalance: async (walletId) => {
    const response = await fetch(`${API_BASE_URL}/get_balance/${walletId}`, {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json'
      }
    });
    
    if (!response.ok) {
      throw new Error(`Failed to get balance: ${response.statusText}`);
    }
    
    return await response.json();
  },
  
  // Send funds to an Ark address
  sendToArkAddress: async (walletId, address, amount) => {
    const response = await fetch(`${API_BASE_URL}/send_to_ark_address`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({
        wallet_id: walletId,
        address: address,
        amount: parseInt(amount, 10)
      })
    });
    
    if (!response.ok) {
      throw new Error(`Failed to send funds: ${response.statusText}`);
    }
    
    return await response.json();
  },
  
  // Request funds from the faucet
  requestFromFaucet: async (onchainAddress, amount) => {
    const response = await fetch(`${API_BASE_URL}/faucet`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({
        onchain_address: onchainAddress,
        amount: parseFloat(amount)
      })
    });
    
    if (!response.ok) {
      throw new Error(`Failed to request from faucet: ${response.statusText}`);
    }
    
    return await response.json();
  },
  
  // Settle funds
  settleFunds: async (walletId, toAddress = null) => {
    const payload = { wallet_id: walletId };
    if (toAddress) {
      payload.to_address = toAddress;
    }
    
    const response = await fetch(`${API_BASE_URL}/settle`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(payload)
    });
    
    if (!response.ok) {
      throw new Error(`Failed to settle funds: ${response.statusText}`);
    }
    
    return await response.json();
  }
};

export default walletService;