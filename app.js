import init, { DeepTutorEngine } from './pkg/core.js';
import { createKnowledgeGraph } from './graph.js';

let engine = null;

const elements = {
    chatHistory: document.getElementById('chat-history'),
    userInput: document.getElementById('user-input'),
    sendBtn: document.getElementById('send-btn'),
    wikiPages: document.getElementById('wiki-pages'),
    openSettings: document.getElementById('open-settings'),
    closeSettings: document.getElementById('close-settings'),
    saveSettings: document.getElementById('save-settings'),
    settingsModal: document.getElementById('settings-modal'),
    overlay: document.getElementById('overlay'),
    ghToken: document.getElementById('gh-token'),
    ghOwner: document.getElementById('gh-owner'),
    ghRepo: document.getElementById('gh-repo'),
    llmKey: document.getElementById('llm-key'),
    toggleGraph: document.getElementById('toggle-graph'),
    graphOverlay: document.getElementById('graph-overlay'),
    closeGraph: document.getElementById('close-graph'),
    pageContent: document.getElementById('page-content'),
    pageTitle: document.getElementById('page-title'),
    pageBody: document.getElementById('page-body'),
    wikiWelcome: document.getElementById('wiki-welcome-msg')
};

async function startApp() {
    try {
        await init();
        console.log("DeepTutor WASM Initialized");
        
        elements.ghToken.value = localStorage.getItem('dt_gh_token') || '';
        elements.ghOwner.value = localStorage.getItem('dt_gh_owner') || '';
        elements.ghRepo.value = localStorage.getItem('dt_gh_repo') || '';
        elements.llmKey.value = localStorage.getItem('dt_llm_key') || '';

        if (elements.ghToken.value && elements.llmKey.value) {
            initializeEngine();
        } else {
            showSettings();
        }
    } catch (err) {
        console.error("Failed to initialize WASM:", err);
        addMessage("tutor", "Error: Failed to load the AI engine. Please ensure the project is built correctly.");
    }
}

function showSettings() {
    elements.settingsModal.style.display = 'block';
    elements.overlay.style.display = 'block';
}

function hideSettings() {
    elements.settingsModal.style.display = 'none';
    elements.overlay.style.display = 'none';
}

function saveSettings() {
    localStorage.setItem('dt_gh_token', elements.ghToken.value);
    localStorage.setItem('dt_gh_owner', elements.ghOwner.value);
    localStorage.setItem('dt_gh_repo', elements.ghRepo.value);
    localStorage.setItem('dt_llm_key', elements.llmKey.value);
    
    initializeEngine();
    hideSettings();
}

async function initializeEngine() {
    const config = {
        token: elements.ghToken.value,
        owner: elements.ghOwner.value,
        repo: elements.ghRepo.value,
        llm_api_key: elements.llmKey.value,
        llm_base_url: "https://api.openai.com/v1",
        llm_model: "gpt-4o"
    };

    try {
        engine = new DeepTutorEngine(
            config.token, config.owner, config.repo,
            config.llm_api_key, config.llm_base_url, config.llm_model
        );
        
        elements.userInput.disabled = false;
        elements.sendBtn.disabled = false;
        elements.toggleGraph.disabled = false;
        addMessage("tutor", "System initialized. Knowledge base linked to " + config.owner + "/" + config.repo);
    } catch (err) {
        console.error("Engine init failed:", err);
        addMessage("tutor", "Failed to initialize engine. Please check your GitHub token.");
    }
}

async function handleSend() {
    const text = elements.userInput.value.trim();
    if (!text || !engine) return;

    elements.userInput.value = '';
    addMessage("user", text);

    const loadingId = addMessage("tutor", "Thinking...");
    
    try {
        const response = await engine.chat(text);
        updateMessage(loadingId, response);
        // Refresh graph if in graph view
        if (elements.graphOverlay.style.display === 'block') {
            loadGraph();
        }
    } catch (err) {
        console.error("Chat error:", err);
        updateMessage(loadingId, "I encountered an error while thinking.");
    }
}

async function loadGraph() {
    if (!engine) return;
    try {
        const graphData = await engine.get_graph_data();
        console.log("Graph Data:", graphData);
        createKnowledgeGraph('graph-container', graphData, (node) => {
            showWikiPage(node.id);
        });
    } catch (err) {
        console.error("Failed to load graph:", err);
    }
}

async function showWikiPage(path) {
    if (!engine) return;
    try {
        elements.graphOverlay.style.display = 'none';
        elements.wikiWelcome.style.display = 'none';
        elements.pageContent.style.display = 'block';
        elements.pageTitle.innerText = path.last();
        elements.pageBody.innerText = "Loading content from GitHub...";

        const content = await engine.get_wiki_content(path);
        // Basic markdown to HTML (could use a library, but let's just use the engine's parser if exposed)
        // For now, just pre-wrap
        elements.pageBody.innerHTML = `<pre style="white-space: pre-wrap; font-family: inherit;">${content}</pre>`;
    } catch (err) {
        console.error("Failed to show page:", err);
        elements.pageBody.innerText = "Error loading page.";
    }
}


function addMessage(role, text) {
    const id = Date.now();
    const div = document.createElement('div');
    div.className = `message ${role}-message`;
    div.id = `msg-${id}`;
    div.innerText = text;
    elements.chatHistory.appendChild(div);
    elements.chatHistory.scrollTop = elements.chatHistory.scrollHeight;
    return id;
}

function updateMessage(id, text) {
    const el = document.getElementById(`msg-${id}`);
    if (el) el.innerText = text;
}

// Event Listeners
elements.openSettings.addEventListener('click', showSettings);
elements.closeSettings.addEventListener('click', hideSettings);
elements.saveSettings.addEventListener('click', saveSettings);
elements.sendBtn.addEventListener('click', handleSend);
elements.toggleGraph.addEventListener('click', () => {
    elements.graphOverlay.style.display = 'block';
    loadGraph();
});
elements.closeGraph.addEventListener('click', () => {
    elements.graphOverlay.style.display = 'none';
});
elements.userInput.addEventListener('keypress', (e) => {
    if (e.key === 'Enter') handleSend();
});

// Polyfill for path split
if (!String.prototype.last) {
    Object.defineProperty(String.prototype, 'last', {
        value: function() {
            return this.split('/').pop().replace('.md', '');
        }
    });
}

// Init
startApp();
