import React, { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface Usuario {
  id: number;
  nombre: string;
  email?: string | null;
}

interface UserSelectorPageProps {
  onUserSelected: (user: Usuario) => void;
}

const UserSelectorPage: React.FC<UserSelectorPageProps> = ({ onUserSelected }) => {
  const [users, setUsers] = useState<Usuario[]>([]);
  const [loading, setLoading] = useState(true);
  const [newUserName, setNewUserName] = useState('');
  const [newUserEmail, setNewUserEmail] = useState('');
  const [error, setError] = useState<string | null>(null);

  const fetchUsers = async () => {
    setLoading(true);
    setError(null);
    try {
      const result = await invoke<Usuario[]>('get_users');
      setUsers(result);
    } catch (e: any) {
      setError('Error al cargar usuarios');
    }
    setLoading(false);
  };

  useEffect(() => {
    fetchUsers();
  }, []);

  const handleSelect = (user: Usuario) => {
    onUserSelected(user);
  };

  const handleAddUser = async () => {
    if (!newUserName.trim() || !newUserEmail.trim()) return;
    setError(null);
    try {
      const user = await invoke<Usuario>('create_user', { nombre: newUserName, email: newUserEmail });
      setNewUserName('');
      setNewUserEmail('');
      setUsers([...users, user]);
      onUserSelected(user);
    } catch (e: any) {
      setError('No se pudo crear el usuario');
    }
  };

  return (
    <div className="user-selector-page" style={{ maxWidth: 600, margin: '2rem auto', padding: '2rem', background: '#fff', borderRadius: 16, boxShadow: '0 2px 16px #0001' }}>
      <h2 style={{ textAlign: 'center', marginBottom: 32 }}>Selecciona tu usuario</h2>
      {loading ? (
        <p>Cargando usuarios...</p>
      ) : users.length === 0 ? (
        <p style={{ textAlign: 'center' }}>No hay usuarios registrados.</p>
      ) : (
        <div style={{ display: 'flex', flexWrap: 'wrap', gap: 16, justifyContent: 'center', marginBottom: 32 }}>
          {users.map((user) => (
            <div key={user.id} style={{ borderRadius: 32, background: '#f3f6fa', padding: '1.2rem 2.5rem', boxShadow: '0 1px 6px #0001', cursor: 'pointer', transition: '0.2s', fontWeight: 600, fontSize: '1.1rem' }} onClick={() => handleSelect(user)}>
              <span role="img" aria-label="user" style={{ marginRight: 8 }}>👤</span>{user.nombre}
              {user.email && <span style={{ display: 'block', fontSize: '0.9rem', color: '#888', marginTop: 4 }}>{user.email}</span>}
            </div>
          ))}
        </div>
      )}
      <div className="add-user-section" style={{ display: 'flex', gap: 8, justifyContent: 'center', marginBottom: 32 }}>
        <input
          type="text"
          placeholder="Nuevo usuario"
          value={newUserName}
          onChange={(e) => setNewUserName(e.target.value)}
          style={{ borderRadius: 20, padding: '0.5rem 1rem', border: '1px solid #ccc', fontSize: '1rem' }}
        />
        <input
          type="email"
          placeholder="Email"
          value={newUserEmail}
          onChange={(e) => setNewUserEmail(e.target.value)}
          style={{ borderRadius: 20, padding: '0.5rem 1rem', border: '1px solid #ccc', fontSize: '1rem' }}
        />
        <button onClick={handleAddUser} style={{ borderRadius: 20, padding: '0.5rem 1.5rem', background: '#1976d2', color: '#fff', border: 'none', fontWeight: 600 }}>Agregar</button>
      </div>
      {users.length > 0 && (
        <table style={{ width: '100%', borderCollapse: 'collapse', background: '#f9fafb', borderRadius: 12, overflow: 'hidden', boxShadow: '0 1px 6px #0001' }}>
          <thead>
            <tr style={{ background: '#e3eaf2' }}>
              <th style={{ padding: '0.7rem' }}>Nombre</th>
              <th style={{ padding: '0.7rem' }}>Email</th>
              <th style={{ padding: '0.7rem' }}>ID</th>
            </tr>
          </thead>
          <tbody>
            {users.map((user) => (
              <tr key={user.id} style={{ textAlign: 'center', cursor: 'pointer' }} onClick={() => handleSelect(user)}>
                <td style={{ padding: '0.7rem', fontWeight: 500 }}>{user.nombre}</td>
                <td style={{ padding: '0.7rem' }}>{user.email}</td>
                <td style={{ padding: '0.7rem', fontFamily: 'monospace' }}>{user.id}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
      {error && <div className="error" style={{ color: '#c00', marginTop: 16, textAlign: 'center' }}>{error}</div>}
    </div>
  );
};

export default UserSelectorPage;
