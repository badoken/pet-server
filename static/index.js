// Fetch and display notes
async function fetchNotes() {
    try {
        const response = await fetch('/notes');
        const notes = await response.json();
        const notesContainer = document.getElementById('notes');
        notesContainer.innerHTML = notes.map(note => `
            <div class="note">
                <h3>${note.name}</h3>
                <p>${note.content}</p>
            </div>
        `).join('');
    } catch (error) {
        console.error('Error fetching notes:', error);
    }
}

// Handle form submission
document.addEventListener('DOMContentLoaded', () => {
    const noteForm = document.getElementById('noteForm');
    noteForm.addEventListener('submit', async (e) => {
        e.preventDefault();
        const formData = new FormData(noteForm);
        const note = {
            name: formData.get('name'),
            content: formData.get('content')
        };

        try {
            await fetch('/note', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify(note)
            });
            noteForm.reset();
            fetchNotes();
        } catch (error) {
            console.error('Error creating note:', error);
        }
    });

    fetchNotes();
});

// Add styles
document.head.innerHTML += `
<style>
    .container {
        max-width: 800px;
        margin: 0 auto;
        padding: 20px;
    }
    .note {
        border: 1px solid #ddd;
        margin: 10px 0;
        padding: 15px;
        border-radius: 5px;
    }
    form {
        display: flex;
        flex-direction: column;
        gap: 10px;
        margin-bottom: 20px;
    }
    input, textarea {
        padding: 8px;
        margin: 5px 0;
    }
    button {
        padding: 10px;
        background: #007bff;
        color: white;
        border: none;
        border-radius: 5px;
        cursor: pointer;
    }
</style>
`;

// Add HTML structure
document.body.innerHTML = `
    <div class="container">
        <h1>Notes</h1>
        <form id="noteForm">
            <input type="text" name="name" placeholder="Note name" required>
            <textarea name="content" placeholder="Note content" required></textarea>
            <button type="submit">Create Note</button>
        </form>
        <div id="notes"></div>
    </div>
`;
