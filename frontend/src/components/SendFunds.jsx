// components/SendFunds.jsx
import React, { useState } from 'react';
import { sendFunds } from '../services/api';

const SendFunds = ({ walletId, selectedAddress }) => {
  const [toAddress, setToAddress] = useState('');
  const [amount, setAmount] = useState('');
  const [sending, setSending] = useState(false);
  const [error, setError] = useState('');
  const [success, setSuccess] = useState('');

  const handleSubmit = async (e) => {
    e.preventDefault();
    
    if (!walletId) {
      setError('No wallet selected');
      return;
    }
    
    if (!toAddress.trim()) {
      setError('Recipient address is required');
      return;
    }
    
    if (!amount || isNaN(parseFloat(amount)) || parseFloat(amount) <= 0) {
      setError('Please enter a valid amount');
      return;
    }
    
    setError('');
    setSuccess('');
    setSending(true);
    
    try {
      const result = await sendFunds(walletId, toAddress, amount);
      setSuccess(`Transaction sent! Transaction ID: ${result.transactionId}`);
      setToAddress('');
      setAmount('');
    } catch (err) {
      setError('Failed to send funds. Please check your inputs and try again.');
    } finally {
      setSending(false);
    }
  };

  return (
    <div className="border rounded-lg p-4 mb-6 bg-white shadow">
      <h2 className="text-xl font-bold mb-4">Send Funds</h2>
      
      <form onSubmit={handleSubmit}>
        <div className="mb-4">
          <label className="block text-sm font-medium mb-1">From</label>
          <input
            type="text"
            value={selectedAddress || ''}
            disabled
            className="w-full px-3 py-2 border rounded-md bg-gray-50"
            placeholder="Select an address from your wallet"
          />
        </div>
        
        <div className="mb-4">
          <label className="block text-sm font-medium mb-1">To Address</label>
          <input
            type="text"
            value={toAddress}
            onChange={(e) => setToAddress(e.target.value)}
            className="w-full px-3 py-2 border rounded-md"
            placeholder="Enter recipient address"
            required
          />
        </div>
        
        <div className="mb-6">
          <label className="block text-sm font-medium mb-1">Amount</label>
          <div className="relative">
            <input
              type="number"
              value={amount}
              onChange={(e) => setAmount(e.target.value)}
              step="0.000001"
              min="0"
              className="w-full px-3 py-2 border rounded-md"
              placeholder="0.00"
              required
            />
            <div className="absolute inset-y-0 right-0 pr-3 flex items-center pointer-events-none">
              <span className="text-gray-500">BTC</span>
            </div>
          </div>
        </div>
        
        <button
          type="submit"
          disabled={sending || !walletId || !selectedAddress}
          className="w-full bg-blue-600 text-white py-2 rounded-md hover:bg-blue-700 disabled:bg-blue-300"
        >
          {sending ? 'Sending...' : 'Send'}
        </button>
        
        {error && (
          <div className="mt-3 text-red-600 text-sm">{error}</div>
        )}
        
        {success && (
          <div className="mt-3 text-green-600 text-sm">{success}</div>
        )}
      </form>
    </div>
  );
};

export default SendFunds;