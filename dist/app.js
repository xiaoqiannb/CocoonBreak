const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

let currentVideos = [];
let currentAnalysis = null;
let currentReverseSearch = null;

document.addEventListener('DOMContentLoaded', () => {
    setupEventListeners();
    setupEventListeners();
});

function setupEventListeners() {
    document.getElementById('fetch-btn').addEventListener('click', fetchRecommendations);
    document.getElementById('analyze-btn').addEventListener('click', analyzeContent);
    document.getElementById('reverse-search-btn').addEventListener('click', generateReverseSearch);
    document.getElementById('start-break-btn').addEventListener('click', startBreakCocoon);
    
    listen('search-progress', handleSearchProgress);
    listen('search-complete', handleSearchComplete);
}

async function fetchRecommendations() {
    const apiKey = document.getElementById('api-key').value;
    
    if (!apiKey) {
        alert('请输入 OpenAI API Key');
        return;
    }
    
    try {
        const btn = document.getElementById('fetch-btn');
        btn.disabled = true;
        btn.innerHTML = '<span class="loading">⏳</span> 获取中...';
        
        currentVideos = await invoke('fetch_recommendations');
        displayVideos(currentVideos);
        
        document.getElementById('analyze-btn').disabled = false;
        btn.disabled = false;
        btn.innerHTML = '获取推荐内容';
        
    } catch (error) {
        console.error('获取推荐失败:', error);
        alert('获取推荐失败: ' + error);
        document.getElementById('fetch-btn').disabled = false;
        document.getElementById('fetch-btn').innerHTML = '获取推荐内容';
    }
}

function displayVideos(videos) {
    const container = document.getElementById('recommendations');
    
    if (videos.length === 0) {
        container.innerHTML = '<p class="placeholder">没有找到推荐视频</p>';
        return;
    }
    
    container.innerHTML = videos.map(video => `
        <div class="video-card">
            <img src="${video.cover}" alt="${video.title}" onerror="this.src='data:image/svg+xml,<svg xmlns=%22http://www.w3.org/2000/svg%22 viewBox=%220 0 100 100%22><rect fill=%22%23ddd%22 width=%22100%22 height=%22100%22/><text fill=%22%23999%22 font-size=%2212%22 x=%2250%25%22 y=%2250%25%22 text-anchor=%22middle%22>无封面</text></svg>'">
            <div class="video-card-content">
                <div class="video-card-title">${video.title}</div>
                <div class="video-card-author">UP主: ${video.author}</div>
                <div class="video-card-stats">
                    ${video.views ? `播放: ${video.views}` : ''}
                </div>
            </div>
        </div>
    `).join('');
}

async function analyzeContent() {
    const apiKey = document.getElementById('api-key').value;
    
    if (!apiKey) {
        alert('请输入 OpenAI API Key');
        return;
    }
    
    try {
        const btn = document.getElementById('analyze-btn');
        btn.disabled = true;
        btn.innerHTML = '<span class="loading">⏳</span> 分析中...';
        
        currentAnalysis = await invoke('analyze_content', { 
            videos: currentVideos,
            apiKey: apiKey
        });
        
        displayAnalysis(currentAnalysis);
        
        document.getElementById('reverse-search-btn').disabled = false;
        btn.disabled = false;
        btn.innerHTML = '分析内容';
        
    } catch (error) {
        console.error('分析失败:', error);
        alert('分析失败: ' + error);
        document.getElementById('analyze-btn').disabled = false;
        document.getElementById('analyze-btn').innerHTML = '分析内容';
    }
}

function displayAnalysis(analysis) {
    const container = document.getElementById('analysis');
    
    container.innerHTML = `
        <div class="analysis-item">
            <h3>📝 内容总结</h3>
            <p>${analysis.summary}</p>
        </div>
        
        <div class="analysis-item">
            <h3>🏷️ 主要话题</h3>
            <div class="topics-list">
                ${analysis.topics.map(topic => `<span class="topic-tag">${topic}</span>`).join('')}
            </div>
        </div>
        
        <div class="analysis-item">
            <h3>⚖️ 偏见分析</h3>
            <p>${analysis.bias_analysis}</p>
        </div>
        
        <div class="analysis-item">
            <h3>📊 信息茧房评分</h3>
            <div class="cocoon-score">${(analysis.cocoon_score * 100).toFixed(1)}%</div>
            <p>${getCocoonScoreDescription(analysis.cocoon_score)}</p>
        </div>
    `;
}

function getCocoonScoreDescription(score) {
    if (score < 0.3) return '🟢 信息获取较为均衡，茧房化程度较低';
    if (score < 0.6) return '🟡 存在一定程度的茧房化，建议多样化内容';
    if (score < 0.8) return '🟠 茧房化程度较高，需要主动打破信息壁垒';
    return '🔴 严重茧房化，强烈建议使用反向搜索功能';
}

async function generateReverseSearch() {
    try {
        const btn = document.getElementById('reverse-search-btn');
        btn.disabled = true;
        btn.innerHTML = '<span class="loading">⏳</span> 生成中...';
        
        currentReverseSearch = await invoke('generate_reverse_search', { 
            analysis: currentAnalysis
        });
        
        displayReverseSearch(currentReverseSearch);
        
        document.getElementById('start-break-btn').disabled = false;
        btn.disabled = false;
        btn.innerHTML = '生成反向搜索';
        
    } catch (error) {
        console.error('生成反向搜索失败:', error);
        alert('生成反向搜索失败: ' + error);
        document.getElementById('reverse-search-btn').disabled = false;
        document.getElementById('reverse-search-btn').innerHTML = '生成反向搜索';
    }
}

function displayReverseSearch(search) {
    const container = document.getElementById('reverse-search');
    
    container.innerHTML = `
        <div class="analysis-item">
            <h3>🔍 反向搜索词</h3>
            <div class="search-terms">
                ${search.search_terms.map(term => `<span class="search-term">${term}</span>`).join('')}
            </div>
        </div>
        
        <div class="analysis-item">
            <h3>💭 生成理由</h3>
            <p>${search.reasoning}</p>
        </div>
        
        <div class="analysis-item">
            <h3>✨ 预期效果</h3>
            <ul class="benefits-list">
                ${search.expected_benefits.map(benefit => `<li>${benefit}</li>`).join('')}
            </ul>
        </div>
    `;
}

async function startBreakCocoon() {
    const apiKey = document.getElementById('api-key').value;
    
    if (!apiKey) {
        alert('请输入 OpenAI API Key');
        return;
    }
    
    if (!currentReverseSearch) {
        alert('请先生成反向搜索策略');
        return;
    }
    
    try {
        const btn = document.getElementById('start-break-btn');
        btn.disabled = true;
        btn.innerHTML = '<span class="loading">⏳</span> 破解中...';
        
        await invoke('search_and_watch', { 
            searchTerms: currentReverseSearch.search_terms,
            apiKey: apiKey
        });
        
        btn.disabled = false;
        btn.innerHTML = '开始破解茧房';
        
    } catch (error) {
        console.error('破解失败:', error);
        alert('破解失败: ' + error);
        document.getElementById('start-break-btn').disabled = false;
        document.getElementById('start-break-btn').innerHTML = '开始破解茧房';
    }
}

function handleSearchProgress(event) {
    const progress = event.payload;
    
    document.getElementById('progress-bar-fill').style.width = `${progress.progress}%`;
    document.getElementById('progress-status').textContent = progress.status;
    
    if (progress.videos_found && progress.videos_found.length > 0) {
        displayProgressVideos(progress.videos_found);
    }
}

function displayProgressVideos(videos) {
    const container = document.getElementById('progress-videos');
    
    container.innerHTML = videos.map(video => `
        <div class="video-card" style="margin-bottom: 10px;">
            <div class="video-card-content">
                <div class="video-card-title">${video.title}</div>
                <div class="video-card-author">UP主: ${video.author}</div>
            </div>
        </div>
    `).join('');
}

function handleSearchComplete(event) {
    document.getElementById('progress-status').textContent = '✅ ' + event.payload;
    alert('信息茧房破解完成！');
}