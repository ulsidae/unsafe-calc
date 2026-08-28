import init, { Calculator } from "./pkg/unsafe_calc.js";

await init();

const calculator = new Calculator();

const expression =
    document.getElementById("expression");

const result =
    document.getElementById("result");

const mode =
    document.getElementById("mode");

function insert(text) {
    expression.value += text;
    expression.focus();
}

function calculate() {
    const input = expression.value.trim();

    if (!input) {
        result.textContent = "0";
        return;
    }

    result.textContent =
        calculator.calculate(input);
}

document
    .querySelectorAll("[data-value]")
    .forEach(button => {

        button.addEventListener("click", () => {
            insert(button.dataset.value);
        });

    });

document
    .querySelectorAll("[data-action]")
    .forEach(button => {

        button.addEventListener("click", () => {

            const action =
                button.dataset.action;

            switch (action) {

                case "calculate":
                    calculate();
                    break;

                case "clear":
                    expression.value = "";
                    result.textContent = "0";
                    break;

                case "backspace":
                    expression.value =
                        expression.value.slice(0, -1);
                    break;

                case "mode": {
                    const next =
                        calculator.mode() === "DEG"
                            ? "rad"
                            : "deg";

                    calculator.set_mode(next);

                    mode.textContent =
                        calculator.mode();

                    break;
                }

                case "memory-add":
                    calculator.memory_add();
                    break;

                case "memory-sub":
                    calculator.memory_sub();
                    break;

                case "memory-recall":
                    insert(
                        calculator.memory_recall()
                    );
                    break;

                case "memory-clear":
                    calculator.memory_clear();
                    break;
            }

        });

    });

expression.addEventListener(
    "keydown",
    event => {

        if (event.key === "Enter") {
            calculate();
        }

    }
);
