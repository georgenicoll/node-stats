import { select_option } from "svelte/internal";

export async function get({ params }) {

    function sleep(ms) {
        return new Promise((resolve) => {
            setTimeout(resolve, ms);
        });
    }

    let body = {
        nodes: [
            { name: "pink", stats: [{ key: "cpu", value: "8%" }, { key: "used-memory", value: "35%" }, { key: "temperature", value: "53 C" }] },
            { name: "black", stats: [{ key: "cpu", value: "23%" }, { key: "used-memory", value: "37%" }, { key: "temperature", value: "54 C" }] },
            { name: "gold", stats: [{ key: "cpu", value: "14%" }, { key: "used-memory", value: "37%" }, { key: "temperature", value: "53 C" }] },
            { name: "red", stats: [{ key: "cpu", value: "10%" }, { key: "used-memory", value: "36%" }, { key: "temperature", value: "55 C" }] },
            { name: "purple", stats: [{ key: "cpu", value: "30%" }, { key: "used-memory", value: "37%" }, { key: "temperature", value: "54 C" }] }
        ]
    };
    await sleep(1000);
    return {
        body: body
    };
}