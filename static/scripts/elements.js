import { Dropdown } from '/node_modules/tw-elements/dist/js/tw-elements.es.min.js';

export function initDropdown(opt) {
    const trigger = document.querySelectorAll(opt);
    [...trigger].forEach((element) => {
        const dropdown = new Dropdown(element);
        const handler = (e) => {
            e.preventDefault();
            dropdown.toggle();
        };
        element.addEventListener('click', handler);
    });
}