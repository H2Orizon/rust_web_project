function editComment(commentId) {
    const form = document.getElementById(`edit-form-${commentId}`);
    if (form) {
        form.classList.remove("d-none");
    }
}

function cancelEdit(id) {
    document.getElementById(`edit-form-${id}`).classList.add("d-none");
    document.getElementById(`comment-text-${id}`).style.display = "block";
}

function submitEdit(event, commentId, itemId) {
    event.preventDefault();
    const content = document.getElementById(`edit-content-${commentId}`).value;

    fetch(`/items/${itemId}/${commentId}/edit_comment`, {
        method: "PATCH",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify({ content })
    }).then(response => {
        if (response.ok) {
            location.reload();
        } else {
            alert("Не вдалося оновити коментар.");
        }
    });
}