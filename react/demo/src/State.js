import { useState } from "react";


function State() {
    const us=["张三","李四","王五","赵六"];
    const [count, setCount] = useState(0);
    const click = () => {
        if(count>=3){
            setCount(0);
            //alert("Please");
        }else{
            setCount(count+1);
        }
      //  setCount(count + 1);
       // setUser({[count]});
        
    }
    return (

        <div>
            <h1>State</h1>
            <button onClick={click}>{count}</button>
            <p>{us[count]}</p>
        </div>
    );

}
export default State;