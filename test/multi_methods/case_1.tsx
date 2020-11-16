class Component extends React.Component {
    clickHandler() {
        console.log('clicked');
    }

    render() {
        return (
            <div onClick={this.clickHandler}>Contents</div>
        )
    }
}
