const BibTex = ( e => {
    
    let elementCode = Elements.bibTexCode;
    let elementModal = Elements.bibTexModal;
    let elementMaskModal = Elements.bibTextMaskModal;

    let get = el => {
        let getElementModal = document.getElementById(elementModal);

        let dataBibtexValue = el.getAttribute('data-bibtex');
        let getBibTex = Licenses.formatBibText(dataBibtexValue);

        document.getElementById(elementCode).innerHTML = getBibTex;

        document.getElementById(elementModal).style.display = 'flex';
        document.getElementById(elementMaskModal).style.display = 'flex';
        
        Effects.heartBeat(getElementModal);
    };

    let copy = e => {
        const codeElement = document.querySelector(`#${ elementCode }`);

        if (codeElement) {
            const range = document.createRange();
            range.selectNodeContents(codeElement);

            const selection = window.getSelection();
            selection.removeAllRanges();

            selection.addRange(range);
            navigator.clipboard.writeText(codeElement.textContent);
        }
    };
    
    let close = e => {
        document.getElementById(elementModal).style.display = 'none';
        document.getElementById(elementMaskModal).style.display = 'none';
    };

    return {
        code: elementCode,
        modal: elementModal,
        mask: elementMaskModal,
        
        copy: () => { return copy() },
        close: () => { return close(); },
        get: (event) => { return get(event) },
    };

})();