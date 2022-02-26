import React, { useEffect, useState } from "react";
import ReactDOM from "react-dom";

const wasm = import("../build/hackathon");

wasm.then((m) => {
    const App = () => {
        const [name, setName] = useState("");
		const [fibResult, setFibResult] = useState();

        const handleChange = (e) => {
            setName(e.target.value);
        };
        const handleClick = () => {
            m.welcome(name);
        };

		const compute = () => {
			setFibResult(m.fib(10));
		}

        return (
            <>
                <div>
                    <h1>Hi there</h1>
                    <button onClick={m.big_computation}>Run Computation</button>
                </div>
                <div>
                    <input type="text" onChange={handleChange} />
                    <button onClick={handleClick}>Say hello!</button>
					<button onClick={compute}>Compute</button>
					<span>{ fibResult }</span>
                </div>
            </>
        );
    };

    ReactDOM.render(<App />, document.getElementById("root"));
});
