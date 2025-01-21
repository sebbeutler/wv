interface TabProps {
    signal: any;
    tabname: string;
    children: any;
}

function Tab({ signal, tabname, children }: TabProps) {
    return (
        <div style={{ display: signal() === tabname ? "block" : "none" }}>
            {children}
        </div>
    );
}

export default Tab;