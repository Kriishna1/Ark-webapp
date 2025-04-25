// src/App.jsx
import React, { useState, useEffect } from 'react';
import { walletService } from './services/api';
import './App.css';

function App() {
  const [activeWallet, setActiveWallet] = useState(null);
  const [walletList, setWalletList] = useState([]);
  const [addresses, setAddresses] = useState(null);
  const [balance, setBalance] = useState(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(null);
  
  // Load stored wallets from localStorage on initial load
  useEffect(() => {
    const storedWallets = localStorage.getItem('ark_wallets');
    if (storedWallets) {
      setWalletList(JSON.parse(storedWallets));
    }
  }, []);
  
  // Save wallets to localStorage when the list changes
  useEffect(() => {
    if (walletList.length > 0) {
      localStorage.setItem('ark_wallets', JSON.stringify(walletList));
    }
  }, [walletList]);
  
  // Fetch wallet details when an active wallet is selected
  useEffect(() => {
    if (activeWallet) {
      fetchWalletDetails(activeWallet);
    }
  }, [activeWallet]);
  
  const fetchWalletDetails = async (walletId) => {
    setLoading(true);
    setError(null);
    
    try {
      // Fetch addresses and balance in parallel
      const [addressData, balanceData] = await Promise.all([
        walletService.getAddresses(walletId),
        walletService.getBalance(walletId)
      ]);
      
      setAddresses(addressData);
      setBalance(balanceData);
    } catch (err) {
      setError(err.message);
      console.error("Error fetching wallet details:", err);
    } finally {
      setLoading(false);
    }
  };
  
  const createNewWallet = async () => {
    setLoading(true);
    setError(null);
    
    try {
      const result = await walletService.createWallet();
      const newWallet = { id: result.wallet_id, createdAt: new Date().toISOString() };
      
      setWalletList([...walletList, newWallet]);
      setActiveWallet(result.wallet_id);
    } catch (err) {
      setError(err.message);
      console.error("Error creating wallet:", err);
    } finally {
      setLoading(false);
    }
  };
  
  const [sendFormData, setSendFormData] = useState({
    address: '',
    amount: '',
  });
  
  const handleSendFormChange = (e) => {
    const { name, value } = e.target;
    setSendFormData({
      ...sendFormData,
      [name]: value
    });
  };
  
  const sendFunds = async (e) => {
    e.preventDefault();
    if (!activeWallet || !sendFormData.address || !sendFormData.amount) return;
    
    setLoading(true);
    setError(null);
    
    try {
      await walletService.sendToArkAddress(
        activeWallet,
        sendFormData.address,
        sendFormData.amount
      );
      
      // Clear form and refresh balance
      setSendFormData({ address: '', amount: '' });
      fetchWalletDetails(activeWallet);
    } catch (err) {
      setError(err.message);
      console.error("Error sending funds:", err);
    } finally {
      setLoading(false);
    }
  };
  
  const [faucetFormData, setFaucetFormData] = useState({
    amount: '0.01'
  });
  
  const handleFaucetFormChange = (e) => {
    const { name, value } = e.target;
    setFaucetFormData({
      ...faucetFormData,
      [name]: value
    });
  };
  
  const requestFromFaucet = async (e) => {
    e.preventDefault();
    if (!addresses) return;
    
    setLoading(true);
    setError(null);
    
    try {
      await walletService.requestFromFaucet(
        addresses.onchain_address,
        faucetFormData.amount
      );
      
      fetchWalletDetails(activeWallet);
    } catch (err) {
      setError(err.message);
      console.error("Error requesting from faucet:", err);
    } finally {
      setLoading(false);
    }
  };
  
  const settleFunds = async () => {
    if (!activeWallet) return;
    
    setLoading(true);
    setError(null);
    
    try {
      await walletService.settleFunds(activeWallet);
      fetchWalletDetails(activeWallet);
    } catch (err) {
      setError(err.message);
      console.error("Error settling funds:", err);
    } finally {
      setLoading(false);
    }
  };
  
  return (
    <div className="app">
      <header>
        <h1>Ark Wallet</h1>
      </header>
      
      <main>
        <div className="wallet-section">
          <h2>Wallets</h2>
          <button onClick={createNewWallet} disabled={loading}>
            Create New Wallet
          </button>
          
          <div className="wallet-list">
            {walletList.map(wallet => (
              <div 
                key={wallet.id} 
                className={`wallet-item ${activeWallet === wallet.id ? 'active' : ''}`}
                onClick={() => setActiveWallet(wallet.id)}
              >
                {wallet.id}
              </div>
            ))}
          </div>
        </div>
        
        {activeWallet && addresses && balance && (
          <div className="wallet-details">
            <h2>Wallet Details</h2>
            
            <div className="address-section">
              <h3>Addresses</h3>
              <div className="address-item">
                <div className="label">Onchain Address:</div>
                <div className="value">{addresses.onchain_address}</div>
              </div>
              <div className="address-item">
                <div className="label">Offchain Address:</div>
                <div className="value">{addresses.offchain_address}</div>
              </div>
            </div>
            
            <div className="balance-section">
              <h3>Balance</h3>
              <div className="balance-group">
                <h4>Offchain</h4>
                <div className="balance-item">
                  <span>Spendable:</span>
                  <span>{balance.offchain_balance.spendable} sats</span>
                </div>
                <div className="balance-item">
                  <span>Expired:</span>
                  <span>{balance.offchain_balance.expired} sats</span>
                </div>
              </div>
              
              <div className="balance-group">
                <h4>Boarding</h4>
                <div className="balance-item">
                  <span>Spendable:</span>
                  <span>{balance.boarding_balance.spendable} sats</span>
                </div>
                <div className="balance-item">
                  <span>Pending:</span>
                  <span>{balance.boarding_balance.pending} sats</span>
                </div>
                <div className="balance-item">
                  <span>Expired:</span>
                  <span>{balance.boarding_balance.expired} sats</span>
                </div>
              </div>
            </div>
            
            <div className="actions-section">
              <div className="send-form">
                <h3>Send Funds</h3>
                <form onSubmit={sendFunds}>
                  <div className="form-group">
                    <label>Recipient Address:</label>
                    <input 
                      type="text" 
                      name="address" 
                      value={sendFormData.address}
                      onChange={handleSendFormChange}
                      required
                    />
                  </div>
                  <div className="form-group">
                    <label>Amount (sats):</label>
                    <input 
                      type="number" 
                      name="amount" 
                      value={sendFormData.amount}
                      onChange={handleSendFormChange}
                      min="1"
                      required
                    />
                  </div>
                  <button type="submit" disabled={loading}>Send</button>
                </form>
              </div>
              
              <div className="faucet-form">
                <h3>Request from Faucet</h3>
                <form onSubmit={requestFromFaucet}>
                  <div className="form-group">
                    <label>Amount (BTC):</label>
                    <input 
                      type="number" 
                      name="amount" 
                      value={faucetFormData.amount}
                      onChange={handleFaucetFormChange}
                      step="0.01"
                      min="0.01"
                      required
                    />
                  </div>
                  <button type="submit" disabled={loading}>Request</button>
                </form>
              </div>
              
              <div className="settle-section">
                <h3>Settle Funds</h3>
                <button onClick={settleFunds} disabled={loading}>
                  Settle to Bitcoin
                </button>
              </div>
            </div>
          </div>
        )}
        
        {loading && <div className="loading-indicator">Loading...</div>}
        {error && <div className="error-message">{error}</div>}
      </main>
    </div>
  );
}

export default App;