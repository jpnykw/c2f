const Component = () => {
    const clickHandler = () => {
        console.log('clicked');
    }

    return (
        <div onClick={this.clickHandler}>
            Contents
        </div>
    )
}
