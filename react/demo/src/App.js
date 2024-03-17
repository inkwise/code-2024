
import State from './State';

function App() {
  return (
    
    <div className="App">
      <State/>
    </div>
  );
}
function Button(){
  return <button onClick={login}>点击</button>;
}
function login(){
  alert('Login');
}

export default App;
