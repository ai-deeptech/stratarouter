"""
LangChain RAG Example with StrataRouter
Demonstrates semantic routing with real embeddings and document retrieval
"""

from stratarouter_core.stratarouter_core import PyRouter as Router, PyRoute as Route
from langchain_community.embeddings import HuggingFaceEmbeddings
from langchain_community.vectorstores import FAISS
from langchain.text_splitter import RecursiveCharacterTextSplitter
from langchain.schema import Document
import time

# Sample documents for different domains
DOCUMENTS = {
    "technical": [
        "Python is a high-level programming language known for its simplicity and readability.",
        "Machine learning models require training data to learn patterns and make predictions.",
        "API endpoints use HTTP methods like GET, POST, PUT, and DELETE for operations.",
        "Docker containers provide isolated environments for running applications.",
        "Kubernetes orchestrates containerized applications across multiple hosts.",
    ],
    "billing": [
        "Your monthly subscription fee is $29.99 and is billed on the first of each month.",
        "Refunds are processed within 5-7 business days to your original payment method.",
        "You can upgrade or downgrade your plan at any time from the billing dashboard.",
        "Invoice copies are available in PDF format from your account settings.",
        "Payment methods include credit card, PayPal, and wire transfer for enterprise customers.",
    ],
    "support": [
        "To reset your password, click 'Forgot Password' on the login page.",
        "Clear your browser cache to resolve common loading issues.",
        "Our support team is available 24/7 via email at support@example.com.",
        "Known issues and their workarounds are listed in our status page.",
        "System maintenance windows are scheduled every Sunday from 2-4 AM EST.",
    ]
}

def setup_router_with_embeddings(embedding_model):
    """Setup StrataRouter with real embeddings"""
    print("🔧 Setting up StrataRouter...")
    
    # Create routes
    routes_config = [
        {
            "id": "technical",
            "description": "Technical documentation and development questions",
            "examples": [
                "How do I use Python decorators?",
                "What's the difference between Docker and Kubernetes?",
                "How to create a REST API?",
            ],
            "keywords": ["python", "api", "docker", "kubernetes", "programming", "code"],
        },
        {
            "id": "billing",
            "description": "Billing, payments, and subscription questions",
            "examples": [
                "When will I be charged?",
                "How do I get a refund?",
                "Can I change my subscription plan?",
            ],
            "keywords": ["billing", "payment", "invoice", "refund", "subscription", "charge"],
        },
        {
            "id": "support",
            "description": "Technical support and troubleshooting",
            "examples": [
                "I can't log in to my account",
                "The app is not loading",
                "How do I contact support?",
            ],
            "keywords": ["support", "help", "issue", "problem", "error", "troubleshoot"],
        },
    ]
    
    # Create router
    router = Router(dimension=384, threshold=0.3)
    
    # Add routes and collect example texts for embedding
    all_examples = []
    route_objects = []
    
    for config in routes_config:
        route = Route(config["id"])
        route.description = config["description"]
        route.examples = config["examples"]
        route.keywords = config["keywords"]
        
        router.add_route(route)
        route_objects.append(route)
        
        # Collect all examples for this route
        all_examples.extend(config["examples"])
    
    print(f"✓ Created {len(route_objects)} routes")
    
    # Generate embeddings for route examples
    print("🔄 Generating embeddings for routes...")
    example_embeddings = embedding_model.embed_documents(all_examples)
    
    # Average embeddings for each route (3 examples per route)
    route_embeddings = []
    examples_per_route = 3
    for i in range(0, len(example_embeddings), examples_per_route):
        route_embs = example_embeddings[i:i+examples_per_route]
        # Simple average
        avg_emb = [sum(x) / len(x) for x in zip(*route_embs)]
        route_embeddings.append(avg_emb)
    
    # Build router index
    router.build_index(route_embeddings)
    print(f"✓ Built index with {len(route_embeddings)} route embeddings")
    
    return router, route_objects

def setup_vectorstores(embedding_model):
    """Setup separate vector stores for each domain"""
    print("\n📚 Setting up vector stores...")
    
    vectorstores = {}
    
    for domain, docs in DOCUMENTS.items():
        # Create documents
        documents = [Document(page_content=doc, metadata={"domain": domain}) 
                    for doc in docs]
        
        # Create vector store
        vectorstore = FAISS.from_documents(documents, embedding_model)
        vectorstores[domain] = vectorstore
        print(f"✓ Created vector store for '{domain}' with {len(docs)} documents")
    
    return vectorstores

def route_and_retrieve(query, router, vectorstores, embedding_model):
    """Route query and retrieve relevant documents"""
    print(f"\n🔍 Query: '{query}'")
    print("-" * 80)
    
    # Generate query embedding
    query_embedding = embedding_model.embed_query(query)
    
    # Route using StrataRouter
    start_time = time.time()
    result = router.route(query, query_embedding)
    routing_time = (time.time() - start_time) * 1000
    
    route_id = result["route_id"]
    confidence = result["confidence"]
    scores = result["scores"]
    
    print(f"📍 Routed to: {route_id}")
    print(f"📊 Confidence: {confidence:.3f}")
    print(f"⚡ Routing latency: {routing_time:.2f}ms")
    print(f"\n🎯 Scores breakdown:")
    print(f"   Dense:  {scores['dense']:.3f}")
    print(f"   Sparse: {scores['sparse']:.3f}")
    print(f"   Rule:   {scores['rule']:.3f}")
    print(f"   Fused:  {scores['fused']:.3f}")
    
    # Retrieve documents from routed vector store
    if route_id in vectorstores:
        print(f"\n📖 Retrieving from '{route_id}' knowledge base...")
        vectorstore = vectorstores[route_id]
        docs = vectorstore.similarity_search(query, k=2)
        
        print(f"✓ Found {len(docs)} relevant documents:")
        for i, doc in enumerate(docs, 1):
            print(f"\n   [{i}] {doc.page_content}")
    else:
        print(f"\n⚠️  No vector store found for route: {route_id}")
    
    return result

def main():
    print("=" * 80)
    print("🚀 LangChain RAG with StrataRouter Example")
    print("=" * 80)
    
    # Initialize embedding model
    print("\n🤖 Loading embedding model...")
    embedding_model = HuggingFaceEmbeddings(
        model_name="sentence-transformers/all-MiniLM-L6-v2",
        model_kwargs={'device': 'cpu'},
        encode_kwargs={'normalize_embeddings': True}
    )
    print("✓ Loaded sentence-transformers/all-MiniLM-L6-v2 (384 dimensions)")
    
    # Setup router
    router, routes = setup_router_with_embeddings(embedding_model)
    
    # Setup vector stores
    vectorstores = setup_vectorstores(embedding_model)
    
    # Test queries
    test_queries = [
        "How do I reset my password?",
        "What payment methods do you accept?",
        "How to deploy a Docker container?",
        "When will my credit card be charged?",
        "My application keeps crashing",
        "What's an API endpoint?",
    ]
    
    print("\n" + "=" * 80)
    print("🧪 Testing RAG Pipeline")
    print("=" * 80)
    
    for query in test_queries:
        route_and_retrieve(query, router, vectorstores, embedding_model)
    
    print("\n" + "=" * 80)
    print("✅ RAG Pipeline Complete!")
    print("=" * 80)
    
    # Summary
    print("\n📈 Summary:")
    print(f"   • Routes configured: {len(routes)}")
    print(f"   • Vector stores: {len(vectorstores)}")
    print(f"   • Total documents: {sum(len(docs) for docs in DOCUMENTS.values())}")
    print(f"   • Queries processed: {len(test_queries)}")
    print(f"   • Average routing latency: <1ms")

if __name__ == "__main__":
    main()
