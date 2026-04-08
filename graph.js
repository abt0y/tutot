/* 
  DeepTutor Knowledge Graph Visualizer (D3.js)
  Visualizes the connections between wiki concepts fetched from the Rust WASM engine.
*/

export function createKnowledgeGraph(containerId, data, onNodeClick) {
    const container = document.getElementById(containerId);
    const width = container.clientWidth;
    const height = container.clientHeight;

    // Clear previous graph
    container.innerHTML = '';

    const svg = d3.select(`#${containerId}`)
        .append("svg")
        .attr("width", width)
        .attr("height", height)
        .attr("viewBox", [0, 0, width, height])
        .attr("style", "max-width: 100%; height: auto;");

    const simulation = d3.forceSimulation(data.nodes)
        .force("link", d3.forceLink(data.links).id(d => d.id).distance(100))
        .force("charge", d3.forceManyBody().strength(-300))
        .force("center", d3.forceCenter(width / 2, height / 2))
        .force("x", d3.forceX())
        .force("y", d3.forceY());

    // Link lines
    const link = svg.append("g")
        .attr("stroke", "#30363d")
        .attr("stroke-opacity", 0.6)
        .selectAll("line")
        .data(data.links)
        .join("line")
        .attr("stroke-width", d => Math.sqrt(d.value || 1));

    // Node groups
    const node = svg.append("g")
        .selectAll("g")
        .data(data.nodes)
        .join("g")
        .call(drag(simulation))
        .on("click", (event, d) => onNodeClick(d));

    // Node circles
    node.append("circle")
        .attr("r", 8)
        .attr("fill", d => d.group === 'concept' ? "#58a6ff" : "#bc8cff")
        .attr("stroke", "#0d1117")
        .attr("stroke-width", 2)
        .attr("style", "cursor: pointer; filter: drop-shadow(0 0 5px rgba(88, 166, 255, 0.4));");

    // Node labels
    node.append("text")
        .attr("x", 12)
        .attr("y", 4)
        .text(d => d.title)
        .attr("fill", "#c9d1d9")
        .attr("font-size", "12px")
        .attr("font-family", "Inter, sans-serif")
        .style("pointer-events", "none")
        .style("text-shadow", "0 0 4px #0d1117");

    simulation.on("tick", () => {
        link
            .attr("x1", d => d.source.x)
            .attr("y1", d => d.source.y)
            .attr("x2", d => d.target.x)
            .attr("y2", d => d.target.y);

        node
            .attr("transform", d => `translate(${d.x},${d.y})`);
    });

    // Zoom behavior
    svg.call(d3.zoom()
        .extent([[0, 0], [width, height]])
        .scaleExtent([0.1, 8])
        .on("zoom", ({transform}) => {
            node.attr("transform", transform);
            link.attr("transform", transform);
        }));

    function drag(simulation) {
        function dragstarted(event) {
            if (!event.active) simulation.alphaTarget(0.3).restart();
            event.subject.fx = event.subject.x;
            event.subject.fy = event.subject.y;
        }

        function dragged(event) {
            event.subject.fx = event.x;
            event.subject.fy = event.y;
        }

        function dragended(event) {
            if (!event.active) simulation.alphaTarget(0);
            event.subject.fx = null;
            event.subject.fy = null;
        }

        return d3.drag()
            .on("start", dragstarted)
            .on("drag", dragged)
            .on("end", dragended);
    }

    return simulation;
}
