// components/WalletInfo.jsx
import React, { useState, useEffect } from 'react';
import { getWalletInfo } from '../services/api';

const WalletInfo = ({ walletId }) => {
  const [walletInfo, setWalletInfo] = useState(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState('');

  useEffect(() => {
    const fetchWalletInfo = async () => {
      if (!walletId) return;
      
      setLoading(true);
      try {
        const info = await getWalletInfo(walletId);
        setWalletInfo(info);
        setError('');
      } catch (err) {
        setError('Failed to load wallet information');
      } finally {
        setLoading(false);
      }
    };

    fetchWalletInfo();
  }, [walletId]);

  if (!walletId) {
    return <div className="text-gray-500">No wallet selected</div>;
  }

  if (loading) {
    return <div className="animate-pulse">Loading wallet info...</div>;
  }

  if (error) {
    return <div className="text-red-500">{error}</div>;
  }

  return (
    <div className="border rounded-lg p-4 mb-6 bg-white shadow">
      <h2 className="text-xl font-bold mb-4">Wallet Information</h2>
      
      {walletInfo && (
        <div className="space-y-2">
          <div className="flex justify-between">
            <span className="text-gray-600">Wallet ID:</span>
            <span className="font-mono">{walletInfo.id}</span>
          </div>
          
          <div className="flex justify-between">
            <span className="text-gray-600">Type:</span>
            <span>{walletInfo.type}</span>
          </div>
          
          <div className="flex justify-between">
            <span className="text-gray-600">Created:</span>
            <span>{new Date(walletInfo.createdAt).toLocaleString()}</span>
          </div>
          
          <div className="flex justify-between">
            <span className="text-gray-600">Status:</span>
            <span className={`font-semibold ${walletInfo.status === 'active' ? 'text-green-600' : 'text-yellow-600'}`}>
              {walletInfo.status}
            </span>
          </div>
        </div>
      )}
    </div>
  );
};

export default WalletInfo;