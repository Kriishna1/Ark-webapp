// components/FaucetRequest.jsx
import React, { useState } from 'react';
import { requestFromFaucet } from '../services/api';

const FaucetRequest = ({ selectedAddress }) => {
  const [network, setNetwork] = useState('testnet');
  const [requesting, setRequesting] = useState(false);
  const [error, setError] = useState('');
  const [success, setSuccess] = useState('');

  const handleRequestFunds = async (e) => {
    e.preventDefault();
    
    if (!selectedAddress) {
      setError('No address selected');
      return;
    }
    
    setError('');
    setSuccess('');
    setRequesting(true);
    
    try {
      const result = await requestFromFaucet(selectedAddress, network);
      setSuccess(`Successfully requested funds! Transaction ID: ${result.transactionId}`);
    } catch (err) {
      setError('Failed to request from faucet. Please try again later.');
    } finally {
      setRequesting(false);
    }
  };

  return (
    <div className="border rounded-lg p-4 mb-6 bg-white shadow">
      <h2 className="text-xl font-bold mb-4">Request Test Funds</h2>
      
      <form onSubmit={handleRequestFunds}>
        <div className="mb-4">
          <label className="block text-sm font-medium mb-1">Address</label>
          <input
            type="text"
            value={selectedAddress || ''}
            disabled
            className="w-full px-3 py-2 border rounded-md bg-gray-50"
            placeholder="Select an address from your wallet"
          />
        </div>
        
        <div className="mb-4">
          <label className="block text-sm font-medium mb-1">Network</label>
          <select
            value={network}
            onChange={(e) => setNetwork(e.target.value)}
            className="w-full px-3 py-2 border rounded-md"
          >
            <option value="testnet">Bitcoin Testnet</option>
            <option value="signet">Bitcoin Signet</option>
            <option value="regtest">Regtest</option>
          </select>
        </div>
        
        <button
          type="submit"
          disabled={requesting || !selectedAddress}
          className="w-full bg-green-600 text-white py-2 rounded-md hover:bg-green-700 disabled:bg-green-300"
        >
          {requesting ? 'Requesting...' : 'Request Funds'}
        </button>
        
        {error && (
          <div className="mt-3 text-red-600 text-sm">{error}</div>
        )}
        
        {success && (
          <div className="mt-3 text-green-600 text-sm">{success}</div>
        )}
        
        <div className="mt-4 text-xs text-gray-500">
          <p>Note: Faucets provide test coins only and have no real-world value. Requests may be rate-limited.</p>
        </div>
      </form>
    </div>
  );
};

export default FaucetRequest;