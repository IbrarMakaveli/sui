
<a name="0x2_display"></a>

# Module `0x2::display`

Defines a Display struct which defines the way an Object
should be displayed. The intention is to keep data as independent
from its display as possible, protecting the development process
and keeping it separate from the ecosystem agreements.

Each of the fields of the Display object should allow for pattern
substitution and filling-in the pieces using the data from the object T.

More entry functions might be added in the future depending on the use cases.


-  [Resource `Display`](#0x2_display_Display)
-  [Struct `DisplayCreated`](#0x2_display_DisplayCreated)
-  [Constants](#@Constants_0)
-  [Function `set_owned`](#0x2_display_set_owned)
-  [Function `empty`](#0x2_display_empty)
-  [Function `share`](#0x2_display_share)
-  [Function `create_and_share`](#0x2_display_create_and_share)
-  [Function `create_with_fields`](#0x2_display_create_with_fields)
-  [Function `set`](#0x2_display_set)
-  [Function `set_multiple`](#0x2_display_set_multiple)
-  [Function `set_internal`](#0x2_display_set_internal)


<pre><code><b>use</b> <a href="">0x1::string</a>;
<b>use</b> <a href="event.md#0x2_event">0x2::event</a>;
<b>use</b> <a href="object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="publisher.md#0x2_publisher">0x2::publisher</a>;
<b>use</b> <a href="transfer.md#0x2_transfer">0x2::transfer</a>;
<b>use</b> <a href="tx_context.md#0x2_tx_context">0x2::tx_context</a>;
<b>use</b> <a href="vec_map.md#0x2_vec_map">0x2::vec_map</a>;
</code></pre>



<a name="0x2_display_Display"></a>

## Resource `Display`

The Display object. Defines the way an object should be
displayed. Display object can only be created and modified with
a PublisherCap, making sure that the rules are set by the owner
of the type.

Each of the display properties should support patterns outside
of the system, making it simpler to customize Display based
on the property values of an Object.
```
// Example of a display object
Display<0x...::capy::Capy> {
fields:
<name, "Capy {{ genes }}">
<link, "https://capy.art/capy/{{ id }}">
<image, "https://api.capy.art/capy/{{ id }}/svg">
<description, "Lovely Capy, one of many">
}
```

Uses only String type due to external-facing nature of the object,
the property names have a priority over their types.


<pre><code><b>struct</b> <a href="display.md#0x2_display_Display">Display</a>&lt;T: key&gt; <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="object.md#0x2_object_UID">object::UID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>fields: <a href="vec_map.md#0x2_vec_map_VecMap">vec_map::VecMap</a>&lt;<a href="_String">string::String</a>, <a href="_String">string::String</a>&gt;</code>
</dt>
<dd>
 Contains fields for display. Currently supported
 fields are: name, link, image and description.
</dd>
</dl>


</details>

<a name="0x2_display_DisplayCreated"></a>

## Struct `DisplayCreated`

Event: emitted when a new Display object has been created for type T.
Type signature of the event corresponds to the type while id serves for
the discovery.


<pre><code><b>struct</b> <a href="display.md#0x2_display_DisplayCreated">DisplayCreated</a>&lt;T: key&gt; <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0x2_display_ENotOwner"></a>

For when T does not belong to package in PublisherCap.


<pre><code><b>const</b> <a href="display.md#0x2_display_ENotOwner">ENotOwner</a>: u64 = 0;
</code></pre>



<a name="0x2_display_EVecLengthMismatch"></a>

For when vectors passed into one of the multiple insert functions
don't match in their lengths.


<pre><code><b>const</b> <a href="display.md#0x2_display_EVecLengthMismatch">EVecLengthMismatch</a>: u64 = 1;
</code></pre>



<a name="0x2_display_set_owned"></a>

## Function `set_owned`

Since the only way to own a Display is before it has been published,
we don't need to perform an authorization check.

Also, the only place it can be used is the function where the Display
object was created; hence values and names are likely to be hardcoded and
vector<u8> is the best type for that purpose.


<pre><code><b>public</b> <b>fun</b> <a href="display.md#0x2_display_set_owned">set_owned</a>&lt;T: key&gt;(d: <a href="display.md#0x2_display_Display">display::Display</a>&lt;T&gt;, name: <a href="">vector</a>&lt;u8&gt;, value: <a href="">vector</a>&lt;u8&gt;): <a href="display.md#0x2_display_Display">display::Display</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="display.md#0x2_display_set_owned">set_owned</a>&lt;T: key&gt;(d: <a href="display.md#0x2_display_Display">Display</a>&lt;T&gt;, name: <a href="">vector</a>&lt;u8&gt;, value: <a href="">vector</a>&lt;u8&gt;): <a href="display.md#0x2_display_Display">Display</a>&lt;T&gt; {
    <a href="display.md#0x2_display_set_internal">set_internal</a>(&<b>mut</b> d, utf8(name), utf8(value));
    d
}
</code></pre>



</details>

<a name="0x2_display_empty"></a>

## Function `empty`

Create an empty Display object. It can either be shared empty of filled
with data right away via cheaper <code>set_owned</code> method.


<pre><code><b>public</b> <b>fun</b> <a href="display.md#0x2_display_empty">empty</a>&lt;T: key&gt;(pub: &<a href="publisher.md#0x2_publisher_Publisher">publisher::Publisher</a>, ctx: &<b>mut</b> <a href="tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="display.md#0x2_display_Display">display::Display</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="display.md#0x2_display_empty">empty</a>&lt;T: key&gt;(pub: &Publisher, ctx: &<b>mut</b> TxContext): <a href="display.md#0x2_display_Display">Display</a>&lt;T&gt; {
    <b>assert</b>!(is_package&lt;T&gt;(pub), <a href="display.md#0x2_display_ENotOwner">ENotOwner</a>);

    <b>let</b> uid = <a href="object.md#0x2_object_new">object::new</a>(ctx);

    <a href="event.md#0x2_event_emit">event::emit</a>(<a href="display.md#0x2_display_DisplayCreated">DisplayCreated</a>&lt;T&gt; {
        id: <a href="object.md#0x2_object_uid_to_inner">object::uid_to_inner</a>(&uid)
    });

    <a href="display.md#0x2_display_Display">Display</a> {
        id: uid,
        fields: <a href="vec_map.md#0x2_vec_map_empty">vec_map::empty</a>()
    }
}
</code></pre>



</details>

<a name="0x2_display_share"></a>

## Function `share`

Share an object after the initialization is complete.


<pre><code><b>public</b> <b>fun</b> <a href="display.md#0x2_display_share">share</a>&lt;T: key&gt;(d: <a href="display.md#0x2_display_Display">display::Display</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="display.md#0x2_display_share">share</a>&lt;T: key&gt;(d: <a href="display.md#0x2_display_Display">Display</a>&lt;T&gt;) {
    <a href="transfer.md#0x2_transfer_share_object">transfer::share_object</a>(d);
}
</code></pre>



</details>

<a name="0x2_display_create_and_share"></a>

## Function `create_and_share`

Create a new empty Display<T> object and share it.


<pre><code><b>public</b> entry <b>fun</b> <a href="display.md#0x2_display_create_and_share">create_and_share</a>&lt;T: key&gt;(pub: &<a href="publisher.md#0x2_publisher_Publisher">publisher::Publisher</a>, ctx: &<b>mut</b> <a href="tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code>entry <b>public</b> <b>fun</b> <a href="display.md#0x2_display_create_and_share">create_and_share</a>&lt;T: key&gt;(pub: &Publisher, ctx: &<b>mut</b> TxContext) {
    <a href="display.md#0x2_display_share">share</a>(<a href="display.md#0x2_display_empty">empty</a>&lt;T&gt;(pub, ctx))
}
</code></pre>



</details>

<a name="0x2_display_create_with_fields"></a>

## Function `create_with_fields`

Create a new Display<T> object with a set of fields.


<pre><code><b>public</b> entry <b>fun</b> <a href="display.md#0x2_display_create_with_fields">create_with_fields</a>&lt;T: key&gt;(pub: &<a href="publisher.md#0x2_publisher_Publisher">publisher::Publisher</a>, fields: <a href="">vector</a>&lt;<a href="_String">string::String</a>&gt;, values: <a href="">vector</a>&lt;<a href="_String">string::String</a>&gt;, ctx: &<b>mut</b> <a href="tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code>entry <b>public</b> <b>fun</b> <a href="display.md#0x2_display_create_with_fields">create_with_fields</a>&lt;T: key&gt;(
    pub: &Publisher, fields: <a href="">vector</a>&lt;String&gt;, values: <a href="">vector</a>&lt;String&gt;, ctx: &<b>mut</b> TxContext
) {
    <b>let</b> len = <a href="_length">vector::length</a>(&fields);
    <b>assert</b>!(len == <a href="_length">vector::length</a>(&values), <a href="display.md#0x2_display_EVecLengthMismatch">EVecLengthMismatch</a>);

    <b>let</b> <a href="display.md#0x2_display">display</a> = <a href="display.md#0x2_display_empty">empty</a>&lt;T&gt;(pub, ctx);
    <b>while</b> (len &gt; 0) {
        <a href="display.md#0x2_display_set_internal">set_internal</a>(&<b>mut</b> <a href="display.md#0x2_display">display</a>, *<a href="_borrow">vector::borrow</a>(&fields, len), *<a href="_borrow">vector::borrow</a>(&values, len));
        len = len - 1;
    };

    <a href="display.md#0x2_display_share">share</a>(<a href="display.md#0x2_display">display</a>)
}
</code></pre>



</details>

<a name="0x2_display_set"></a>

## Function `set`

Sets a custom <code>name</code> field with the <code>value</code>.


<pre><code><b>public</b> entry <b>fun</b> <a href="display.md#0x2_display_set">set</a>&lt;T: key&gt;(pub: &<a href="publisher.md#0x2_publisher_Publisher">publisher::Publisher</a>, d: &<b>mut</b> <a href="display.md#0x2_display_Display">display::Display</a>&lt;T&gt;, name: <a href="_String">string::String</a>, value: <a href="_String">string::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code>entry <b>public</b> <b>fun</b> <a href="display.md#0x2_display_set">set</a>&lt;T: key&gt;(pub: &Publisher, d: &<b>mut</b> <a href="display.md#0x2_display_Display">Display</a>&lt;T&gt;, name: String, value: String) {
    <b>assert</b>!(is_package&lt;T&gt;(pub), <a href="display.md#0x2_display_ENotOwner">ENotOwner</a>);
    <a href="display.md#0x2_display_set_internal">set_internal</a>(d, name, value)
}
</code></pre>



</details>

<a name="0x2_display_set_multiple"></a>

## Function `set_multiple`

Sets multiple <code>fields</code> with <code>values</code>.


<pre><code><b>public</b> entry <b>fun</b> <a href="display.md#0x2_display_set_multiple">set_multiple</a>&lt;T: key&gt;(pub: &<a href="publisher.md#0x2_publisher_Publisher">publisher::Publisher</a>, d: &<b>mut</b> <a href="display.md#0x2_display_Display">display::Display</a>&lt;T&gt;, fields: <a href="">vector</a>&lt;<a href="_String">string::String</a>&gt;, values: <a href="">vector</a>&lt;<a href="_String">string::String</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code>entry <b>public</b> <b>fun</b> <a href="display.md#0x2_display_set_multiple">set_multiple</a>&lt;T: key&gt;(
    pub: &Publisher, d: &<b>mut</b> <a href="display.md#0x2_display_Display">Display</a>&lt;T&gt;, fields: <a href="">vector</a>&lt;String&gt;, values: <a href="">vector</a>&lt;String&gt;
) {
    <b>let</b> len = <a href="_length">vector::length</a>(&fields);
    <b>assert</b>!(is_package&lt;T&gt;(pub), <a href="display.md#0x2_display_ENotOwner">ENotOwner</a>);
    <b>assert</b>!(len == <a href="_length">vector::length</a>(&values), <a href="display.md#0x2_display_EVecLengthMismatch">EVecLengthMismatch</a>);

    <b>while</b> (len &gt; 0) {
        <a href="display.md#0x2_display_set_internal">set_internal</a>(d, *<a href="_borrow">vector::borrow</a>(&fields, len), *<a href="_borrow">vector::borrow</a>(&values, len));
        len = len - 1;
    };
}
</code></pre>



</details>

<a name="0x2_display_set_internal"></a>

## Function `set_internal`

Private method for inserting fields without security checks.


<pre><code><b>fun</b> <a href="display.md#0x2_display_set_internal">set_internal</a>&lt;T: key&gt;(d: &<b>mut</b> <a href="display.md#0x2_display_Display">display::Display</a>&lt;T&gt;, name: <a href="_String">string::String</a>, value: <a href="_String">string::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="display.md#0x2_display_set_internal">set_internal</a>&lt;T: key&gt;(d: &<b>mut</b> <a href="display.md#0x2_display_Display">Display</a>&lt;T&gt;, name: String, value: String) {
    <a href="vec_map.md#0x2_vec_map_insert">vec_map::insert</a>(&<b>mut</b> d.fields, name, value)
}
</code></pre>



</details>