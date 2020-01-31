function printCv() {
    const doc = new jsPDF();
    //const cv = document.querySelector('.cv');

    doc.html(document.body, {
        callback: function (doc) {
            doc.save();
        }
    })
}