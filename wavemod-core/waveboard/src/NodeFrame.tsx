import "./NodeFrame.css";

interface NodeProps {
    name: string,
    children: any | null,
}

export default function NodeFrame({ name, children }: NodeProps) {
    return (
        <div class="node-frame" data-node-name={name}>
            <div class="node-toolbox">
                <button>Schema</button>
                <button>Code</button>
                <button>Q</button>
            </div>
            {children}
        </div>
    );
}
