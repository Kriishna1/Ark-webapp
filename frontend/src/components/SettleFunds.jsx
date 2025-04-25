// components/SettleFunds.jsx
import React, { useState, useEffect } from 'react';
import { settleFunds } from '../services/api';

const SettleFunds = ({ walletId }) => {
  const [pendingTransactions, setPendingTransactions] = useState([]);
  const [selectedTransactions, setSelectedTransactions] = useState([]);
  const [settling, setSettling] = useState(false);
  const [error, setError] = useState('');
  const [success, setSuccess] = useState('');

  // Fetch pending transactions - this would be implemented in a real app
  useEffect(() => {
    // Mock data for demonstration
    if (walletId) {
      setPendingTransactions([
        { id: 'tx1', amount: 0.01, timestamp: new Date().toISOString(), status: 'pending' },
        { id: 'tx2', amount: 0.005, timestamp: new Date().toISOString(), status: 'pending' }
      ]);
    } else {
      setPendingTransactions([]);
    }
  }, [walletId]);

  const handleCheckboxChange = (txId) => {
    setSelectedTransactions(prev => 
      prev.includes(txId)
        ? prev.filter(id => id !== txId)
        : [...prev, txId]
    );
  };

  const handleSettleFunds = async () => {
    if (!walletId || selectedTransactions.length === 0) {
      return;
    }
    
    setError('');
    setSuccess('');
    setSettling(true);
    
    try {
      await settleFunds(walletId, selectedTransactions);
      setSuccess('Transactions successfully settled!');
      
      // Remove settled transactions from the list
      setPendingTransactions(prev => 
        prev.filter(tx => !selectedTransactions.includes(tx.id))
      );
      setSelectedTransactions([]);
    } catch (err) {
      setError('Failed to settle transactions. Please try again.');
    } finally {
      setSettling(false);
    }
  };

  const formatDate = (dateString) => {
    return new Date(dateString).toLocaleString();
  };

  if (!walletId) {
    return <div className="text-gray-500">No wallet selected</div>;
  }

  return (
    <div className="border rounded-lg p-4 mb-6 bg-white shadow">
      <h2 className="text-xl font-bold mb-4">Settle Pending Transactions</h2>
      
      {pendingTransactions.length === 0 ? (
        <p className="text-gray-500">No pending transactions to settle</p>
      ) : (
        <>
          <div className="mb-4">
            <ul className="divide-y">
              {pendingTransactions.map(tx => (
                <li key={tx.id} className="py-3 flex items-center">
                  <input
                    type="checkbox"
                    id={`tx-${tx.id}`}
                    checked={selectedTransactions.includes(tx.id)}
                    onChange={() => handleCheckboxChange(tx.id)}
                    className="mr-3"
                  />
                  <label htmlFor={`tx-${tx.id}`} className="flex-grow cursor-pointer">
                    <div className="flex justify-between">
                      <span className="font-mono text-sm">{tx.id}</span>
                      <span className="font-semibold">{tx.amount} BTC</span>
                    </div>
                    <div className="text-xs text-gray-500">{formatDate(tx.timestamp)}</div>
                  </label>
                </li>
              ))}
            </ul>
          </div>
          
          <button
            onClick={handleSettleFunds}
            disabled={settling || selectedTransactions.length === 0}
            className="w-full bg-purple-600 text-white py-2 rounded-md hover:bg-purple-700 disabled:bg-purple-300"
          >
            {settling ? 'Settling...' : `Settle ${selectedTransactions.length} Transaction(s)`}
          </button>
          
          {error && (
            <div className="mt-3 text-red-600 text-sm">{error}</div>
          )}
          
          {success && (
            <div className="mt-3 text-green-600 text-sm">{success}</div>
          )}
        </>
      )}
    </div>
  );
};

export default SettleFunds;