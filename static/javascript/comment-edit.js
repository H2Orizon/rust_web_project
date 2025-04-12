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

async function submitEdit(event, commentId, itemId) {
    event.preventDefault();
    const content = document.getElementById(`edit-content-${commentId}`).value;

    const response = await fetch(`/items/${itemId}/${commentId}/edit_comment`, {
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify({ content })
    });

    if (response.ok) {
        const data = await response.json();
        document.getElementById(`comment-text-${commentId}`).textContent = data.updated_content;
        cancelEdit(commentId);
    } else {
        alert("Не вдалося оновити коментар.");
    }
}