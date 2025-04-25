// components/WalletCreator.jsx
import React, { useState } from 'react';
import { createWallet } from '../services/api';

const WalletCreator = ({ onWalletCreated }) => {
  const [walletType, setWalletType] = useState('standard');
  const [isCreating, setIsCreating] = useState(false);
  const [error, setError] = useState('');

  const handleCreateWallet = async (e) => {
    e.preventDefault();
    setIsCreating(true);
    setError('');
    
    try {
      const wallet = await createWallet(walletType);
      onWalletCreated(wallet);
    } catch (err) {
      setError('Failed to create wallet. Please try again.');
    } finally {
      setIsCreating(false);
    }
  };

  return (
    <div className="border rounded-lg p-4 mb-6 bg-white shadow">
      <h2 className="text-xl font-bold mb-4">Create New Wallet</h2>
      <form onSubmit={handleCreateWallet}>
        <div className="mb-4">
          <label className="block text-sm font-medium mb-1">Wallet Type</label>
          <select
            value={walletType}
            onChange={(e) => setWalletType(e.target.value)}
            className="w-full px-3 py-2 border rounded-md"
          >
            <option value="standard">Standard</option>
            <option value="hardware">Hardware</option>
            <option value="multisig">Multi-signature</option>
          </select>
        </div>
        
        <button 
          type="submit" 
          disabled={isCreating}
          className="w-full bg-blue-600 text-white py-2 rounded-md hover:bg-blue-700 disabled:bg-blue-300"
        >
          {isCreating ? 'Creating...' : 'Create Wallet'}
        </button>
        
        {error && <p className="mt-2 text-red-600 text-sm">{error}</p>}
      </form>
    </div>
  );
};

export default WalletCreator;