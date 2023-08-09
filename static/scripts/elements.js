import { Dropdown } from '/node_modules/tw-elements/dist/js/tw-elements.es.min.js';

export function initDropdown(opt) {
    const trigger = document.querySelectorAll(opt);
    const triggers = [].slice.call(trigger);
    triggers.forEach((el) => {
        const dropdown = new Dropdown(el);
        const handler = (e) => {
            e.preventDefault();
            dropdown.toggle();
        };
        el.addEventListener('click', handler);
    });
}
