const { invoke } = window.__TAURI__.tauri;

let num1;
let num2;
let operation;
let result;

async function calculate() {
  result.textContent = await invoke("calculate", {
    number1: num1.value,
    number2: num2.value,
    operation: operation.value
  });
}

window.addEventListener("DOMContentLoaded", () => {
  num1 = document.querySelector("#txtNum1");
  num2 = document.querySelector("#txtNum2");
  operation = document.querySelector("#slcOperation");
  result = document.querySelector("#lblResult");
  document
    .querySelector("#btnCalculate")
    .addEventListener("click", () => calculate());
});
