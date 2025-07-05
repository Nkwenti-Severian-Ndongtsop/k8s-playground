let token = null;

// Use the same hostname but different port for backend
const BACKEND_BASE_URL = `http://${window.location.hostname}:30081`;

document.getElementById('register-form').onsubmit = async (e) => {
  e.preventDefault();
  const form = e.target;
  const res = await fetch(`${BACKEND_BASE_URL}/register`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
      username: form.username.value,
      password: form.password.value
    })
  });
  if (res.status === 201) {
    // Registration successful, redirect to login page
    window.location.href = 'login.html';
  } else {
    alert('Registration failed!');
  }
};
document.getElementById('login-form').onsubmit = async (e) => {
  e.preventDefault();
  const form = e.target;
  const res = await fetch(`${BACKEND_BASE_URL}/login`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
      username: form.username.value,
      password: form.password.value
    })
  });
  if (res.ok) {
    const data = await res.json();
    token = data.token;
    localStorage.setItem('token', data.token);
    // Redirect to profile page
    window.location.href = 'profile.html';
  } else {
    alert('Login failed!');
  }
};
async function showWelcome() {
  const res = await fetch(`${BACKEND_BASE_URL}/me`, {
    headers: { 'Authorization': 'Bearer ' + token }
  });
  const data = await res.json();
  document.getElementById('forms').style.display = 'none';
  document.getElementById('welcome').style.display = 'block';
  document.getElementById('welcome').innerText = 'Welcome, ' + data.username;
}

// Login form handler for index.html
if (document.getElementById('loginForm')) {
  document.getElementById('loginForm').addEventListener('submit', async function(e) {
    e.preventDefault();
    const username = document.getElementById('username').value;
    const password = document.getElementById('password').value;
    const res = await fetch(`${BACKEND_BASE_URL}/login`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ username, password })
    });
    const msg = document.getElementById('loginMessage');
    if (res.ok) {
      const data = await res.json();
      msg.textContent = 'Login successful!';
      msg.style.color = 'green';
      localStorage.setItem('token', data.token);
      // Redirect to profile page
      window.location.href = 'profile.html';
    } else {
      msg.textContent = 'Login failed. Check your credentials.';
      msg.style.color = 'red';
    }
  });
} 